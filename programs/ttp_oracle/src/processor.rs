use crate::{
  PUBLIC_KEY_LEN,
  instruction::{ OracleInstruction },
  request::Request,
  response::Response,
};
use solana_program::{
  account_info::{ next_account_info, AccountInfo },
  entrypoint::ProgramResult,
  instruction::Instruction,
  program_error::ProgramError,
  program_pack::Pack,
  program::invoke,
  pubkey::Pubkey,
};
use arrayref::{ array_ref, array_mut_ref, mut_array_refs };

pub const CALL_BACK_DETERMINANT: u8 = 255;

pub struct Processor {}
impl Processor {

  // process instructions
  pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = OracleInstruction::unpack(input)?;

    match instruction {
      OracleInstruction::CreateRequest { request } => Self::process_create_request(accounts, &request),
      OracleInstruction::HandleResponse(response) => Ok(())
    }
  }

  /**
   * TODO serialize the Request from CreateRequestData and add it to the oracle data account
   * TODO allow handling more than 1 request. Current implementation simply overwrites the 
   *  entire account data buffer
   */
  pub fn process_create_request(accounts: &[AccountInfo], request: &Request) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let oracle_account = next_account_info(accounts_iter)?;

    let mut data = oracle_account.try_borrow_mut_data()?;
    let mut serialized_request = [0u8; Request::LEN];
    request.pack_into_slice(&mut serialized_request);

    let dst = array_mut_ref![data, 0, Request::LEN];
    // overwrite account data
    dst.copy_from_slice(&serialized_request);

    Ok(())
  }

  pub fn process_handle_response(accounts: &[AccountInfo], response: &Response) -> ProgramResult {
    // clear the account data
    let accounts_iter = &mut accounts.iter();
    let oracle_account = next_account_info(accounts_iter)?;
    let mut data = oracle_account.try_borrow_mut_data()?;
    data.copy_from_slice(&[0u8; Request::LEN]);
    // send a cross program invocation to the second account
    let client_program_account = next_account_info(accounts_iter)?;
    let data = &mut [0u8; (Response::LEN + 1 as usize)];
    let determinant_size: usize = 1;
    response.pack_into_slice(&mut data[1..(Response::LEN + determinant_size)]);
    let determinant = array_mut_ref![data, 0, 1];
    *determinant = CALL_BACK_DETERMINANT.to_le_bytes();
    let accounts = vec![];
    let ix = Instruction {
      program_id: *client_program_account.key,
      accounts,
      data: data.to_vec()
    };
    invoke(&ix, &[client_program_account.clone()]);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PUBLIC_KEY_LEN;
  use super::*;
  use std::format;
  use generic_array::GenericArray;
  use solana_program::{
    clock::Epoch,
    info,
    instruction::Instruction,
    program_stubs,
  };
  use crate::request::{
    GetArgs,
    GetParams,
    JsonParseArgs,
    Task
  };

  // test program id for ttp-oralce program
  const TTP_ORACLE_PROGRAM_ID: Pubkey = Pubkey::new_from_array([1u8; PUBLIC_KEY_LEN]);
  // test program id for the client program that consumes ttp-oracle
  const CLIENT_PROGRAM_ID: Pubkey = Pubkey::new_from_array([2u8; PUBLIC_KEY_LEN]);

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

  fn invoke_client<'a>(account_infos: &[AccountInfo<'a>], input: &[u8]) -> ProgramResult {
    let res_data = array_ref![input, 0, 16];
    // read the data sent back (le u128)
    let price = u128::from_le_bytes(*res_data);
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

    return Request {
      tasks: [get_task, json_parse_task, uint_128_task],
      call_back_program: Pubkey::new_unique()
    };
  }

  #[test]
  fn test_process_create_request() {
    let program_id = Pubkey::default();
    let oracle_id = Pubkey::default();
    let mut lamports = 0;
    // account data buffer with the size of a request
    let request = build_request();
    let mut data_buffer = vec![1; Request::LEN];
    let account = AccountInfo::new(&oracle_id, false, true, &mut lamports, &mut data_buffer, &program_id, false, Epoch::default());
    let accounts = vec![account];


    let ret = Processor::process_create_request(&accounts, &request);
    assert!(ret.is_ok());
    
    let account_data = accounts[0].data.borrow();

    let deserialized_request: Request = Request::unpack_from_slice(&account_data).unwrap();

    assert_eq!(deserialized_request, request);
  }

  #[test]
  fn test_process_handle_response() {
    let program_id = Pubkey::default();
    let oracle_id = Pubkey::default();
    let mut lamports = 0;
    let mut lamports2 = 0;
    // account data buffer with the size of a request
    let request = build_request();
    let mut data_buffer = [1u8; Request::LEN];
    request.pack_into_slice(&mut data_buffer);
    let callback_program_account = AccountInfo::new(&CLIENT_PROGRAM_ID, false, false, &mut lamports2, &mut [], &program_id, true, Epoch::default());
    let oracle_data_account = AccountInfo::new(&oracle_id, false, true, &mut lamports, &mut data_buffer, &program_id, false, Epoch::default());
    let accounts = vec![oracle_data_account, callback_program_account];

    let response_val: u128 = 15439;
    let response = Response {
      data: response_val.to_le_bytes()
    };
    let ret = Processor::process_handle_response(&accounts, &response);
    assert!(ret.is_ok());

    // it should clear the account data
    let account_data = accounts[0].data.borrow();
    let account_data_slice = array_ref![account_data, 0, Request::LEN];
    assert_eq!(account_data_slice, &[0u8; Request::LEN]);

    // verify it returned the response from the cross program stub
  }
}