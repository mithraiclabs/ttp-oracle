use crate::{
  instruction::{ OracleInstruction },
  request::{ Request, REQUEST_QUEUE_SIZE },
  response::Response,
  oracle_account::OracleAccount
};
use solana_program::{
  account_info::{ next_account_info, AccountInfo },
  entrypoint::ProgramResult,
  instruction::Instruction,
  program_pack::Pack,
  program::invoke,
  pubkey::Pubkey,
};
use arrayref::{ array_mut_ref };

pub const CALLBACK_DETERMINANT: u8 = 255;
pub const CALLBACK_DETERMINANT_SIZE: usize = 1;

pub struct Processor {}
impl Processor {
  /// process instructions
  pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = OracleInstruction::unpack(input)?;

    match instruction {
      OracleInstruction::CreateRequest { request } => Self::process_create_request(accounts, request),
      OracleInstruction::HandleResponse(response) => Self::process_handle_response(accounts, response),
    }
  }

  /**
   * Find and insert Request in the first open slot on the OracleAccount's RequestQueue
   */
  pub fn process_create_request(accounts: &[AccountInfo], mut request: Request) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let oracle_account = next_account_info(accounts_iter)?;
    
    let mut account_data = oracle_account.data.borrow_mut();
    let oracle_data = OracleAccount::unpack(&account_data)?;
    for i in 0..REQUEST_QUEUE_SIZE {
      // loop to find the first empty request
      if oracle_data.request_queue.requests[i].is_none() {
        let offset = Request::LEN * i;
        let dest = array_mut_ref![account_data, offset, Request::LEN];
        request.index  = i as u8;
        Request::pack(request, dest)?;
        break;
      }
    }

    Ok(())
  }

  /// Convert the response data into data bufer to be sent to the Caller Program
  pub fn process_handle_response(accounts: &[AccountInfo], response: Response) -> ProgramResult {
    // clear the account data
    let accounts_iter = &mut accounts.iter();
    let oracle_account = next_account_info(accounts_iter)?;
    let mut account_data = oracle_account.data.borrow_mut();

    // delete the Request that the Response is for
    let offset = response.request_queue_index as usize * Request::LEN;
    let request_to_delete = array_mut_ref![account_data, offset, Request::LEN];
    request_to_delete.copy_from_slice(&[0u8; Request::LEN]);

    // send a cross program invocation to the second account
    let client_program_account = next_account_info(accounts_iter)?;
    let data = &mut [0u8; Response::LEN];
    Response::pack(response, data)?;
    let accounts = vec![];
    let ix = Instruction {
      program_id: *client_program_account.key,
      accounts,
      data: data.to_vec()
    };

    invoke(&ix, &[client_program_account.clone()])
  }
}

#[cfg(test)]
mod tests {
  use crate::{ 
    instruction::*,
    PUBLIC_KEY_LEN,
    request::{ GetArgs,
      GetParams,
      JsonParseArgs,
      Task,
      Request, 
      RequestQueue 
    },
  };
  use super::*;
  use generic_array::GenericArray;
  use solana_program::{
    instruction::{ AccountMeta, Instruction },
    program_stubs,
    program_error::ProgramError,
  };
  use solana_sdk::account::{
    create_is_signer_account_infos, Account
  };
  use arrayref::{ array_ref };

  // test program id for ttp-oralce program
  const TTP_ORACLE_PROGRAM_ID: Pubkey = Pubkey::new_from_array([1u8; PUBLIC_KEY_LEN]);
  // test program id for the client program that consumes ttp-oracle
  const CLIENT_PROGRAM_ID: Pubkey = Pubkey::new_from_array([2u8; PUBLIC_KEY_LEN]);


  fn do_process_instruction(
    instruction: Instruction,
    accounts: Vec<&mut Account>,
) -> ProgramResult {
    let mut meta = instruction
        .accounts
        .iter()
        .zip(accounts)
        .map(|(account_meta, account)| (&account_meta.pubkey, account_meta.is_signer, account))
        .collect::<Vec<_>>();

    let account_infos = create_is_signer_account_infos(&mut meta);
    Processor::process(&instruction.program_id, &account_infos, &instruction.data)
}

  // stub the cross program invocation.
  // This was mainly ripped from solana-program-library repo in stake-pool
  struct TestSyscallStubs {}
  impl program_stubs::SyscallStubs for TestSyscallStubs {
      fn sol_invoke_signed(
          &self,
          instruction: &Instruction,
          account_infos: &[AccountInfo],
          _signers_seeds: &[&[&[u8]]],
      ) -> ProgramResult {

          let mut new_account_infos = vec![];
          for account_info in account_infos.iter() {
            if *account_info.key != CLIENT_PROGRAM_ID {
              new_account_infos.push(account_info.clone())
            }
          }

          match instruction.program_id {
              TTP_ORACLE_PROGRAM_ID => Ok(()), 
              CLIENT_PROGRAM_ID => invoke_client(&new_account_infos, &instruction.data),
              _ => Err(ProgramError::IncorrectProgramId)
          }
      }
  }

  fn invoke_client<'a>(_account_infos: &[AccountInfo<'a>], input: &[u8]) -> ProgramResult {
    let res_data = array_ref![input, 0, 16];
    // read the data sent back (le u128)
    let _price = u128::from_le_bytes(*res_data);
    // return the response for testing purposes
    Ok(())
  }

  fn setup_syscall_stubs() {
      use std::sync::Once;
      static ONCE: Once = Once::new();

      ONCE.call_once(|| {
          program_stubs::set_syscall_stubs(Box::new(TestSyscallStubs {}));
      });
    }

  fn build_request() -> Request {
    // TODO DRY up this set up as it duplicates set up in request.rs
    let url_bytes = b"https://ftx.us/api/markets/BTC/USD";
    let path_bytes = b"result.price";
    let json_args = JsonParseArgs {
      path: *path_bytes
    };
    let params = GetParams {
      get: *GenericArray::from_slice(url_bytes)
    };
    let args = GetArgs {
      params: params
    };
    let get_task = Task::HttpGet(args);
    let json_parse_task = Task::JsonParse(json_args);
    let uint_128_task = Task::Uint128;

    Request {
      tasks: [get_task, json_parse_task, uint_128_task],
      call_back_program: Pubkey::new(&[3u8; PUBLIC_KEY_LEN]),
      index: 0,
    }
  }

  #[test]
  fn test_process_create_request() {
    let program_id = Pubkey::new_unique();
    let oracle_id = Pubkey::new_unique();
    // account data buffer with the size of a request
    let request = build_request();
    let mut oracle_account = Account::new(0, OracleAccount::LEN, &program_id);

    let ix = create_request(&program_id, &oracle_id, request).unwrap();

    do_process_instruction(ix, vec![&mut oracle_account]).unwrap();
    
    let deserialized_oracle_account = OracleAccount::unpack(&oracle_account.data).unwrap();
    
    let request = build_request();
    let request_queue = RequestQueue {
      requests: Box::new([Some(request), None, None, None, None, None, None, None, None, None]),
    };
    let oracle_account = OracleAccount {
      request_queue,
    };
    assert_eq!(deserialized_oracle_account, oracle_account);
  }

  #[test]
  fn test_process_create_two_requests() {
    let program_id = Pubkey::new_unique();
    let oracle_id = Pubkey::new_unique();
    // account data buffer with the size of a request
    let request = build_request();
    let mut account = Account::new(0, OracleAccount::LEN, &program_id);
    
    let ix = create_request(&program_id, &oracle_id, request).unwrap();
    
    do_process_instruction(ix, vec![&mut account]).unwrap();
    
    let deserialized_oracle_account = OracleAccount::unpack(&account.data).unwrap();
    
    let request = build_request();
    let request_queue = RequestQueue {
      requests: Box::new([Some(request), None, None, None, None, None, None, None, None, None]),
    };
    let oracle_account_data = OracleAccount {
      request_queue,
    };
    assert_eq!(deserialized_oracle_account, oracle_account_data);
    
    let request = build_request();
    let ix = create_request(&program_id, &oracle_id, request).unwrap();
    do_process_instruction(ix, vec![&mut account]).unwrap();
    let deserialized_oracle_account = OracleAccount::unpack(&account.data).unwrap();
    let request1 = build_request();
    let mut request2 = build_request();
    request2.index = 1;
    let request_queue = RequestQueue {
      requests: Box::new([Some(request1), Some(request2), None, None, None, None, None, None, None, None]),
    };
    let oracle_account_data = OracleAccount {
      request_queue,
    };


    assert_eq!(deserialized_oracle_account, oracle_account_data);
  }

  #[test]
  fn test_process_handle_response() {
    setup_syscall_stubs();
    let system_program = Pubkey::default();
    let program_id = Pubkey::new_unique();
    let oracle_id = Pubkey::new_unique();
    let mut account = Account::new(0, OracleAccount::LEN, &program_id);
    let mut client_program_account = Account::new(0, 0, &system_program);
    let request1 = build_request();
    let mut request2 = build_request();
    request2.index = 1;
    let request_queue = RequestQueue {
      requests: Box::new([Some(request1), Some(request2), None, None, None, None, None, None, None, None]),
    };
    let oracle_account_data = OracleAccount {
      request_queue,
    };
    OracleAccount::pack(oracle_account_data, &mut account.data).unwrap();
    
    let response_val: u128 = 15439;
    let response = Response {
      data: response_val.to_le_bytes(),
      request_queue_index: 1,
    };
    let instruction = OracleInstruction::HandleResponse(response);
    let mut data = [0u8; OracleInstruction::LEN];
    OracleInstruction::pack(instruction, &mut data).unwrap();

    let ix = Instruction {
      program_id: program_id,
      accounts: vec![AccountMeta::new(oracle_id, false), AccountMeta::new(CLIENT_PROGRAM_ID, false)],
      data: data.to_vec(),
    };
    do_process_instruction(ix, vec![&mut account, &mut client_program_account]).unwrap();
    let deserialized_oracle_account = OracleAccount::unpack(&account.data).unwrap();

    let request = build_request();
    let request_queue = RequestQueue {
      requests: Box::new([Some(request), None, None, None, None, None, None, None, None, None]),
    };
    let expected_oracle_account_data = OracleAccount {
      request_queue,
    };

    assert_eq!(deserialized_oracle_account, expected_oracle_account_data);

  }
}