use std::format;
use solana_program::{
    account_info::{ next_account_info, AccountInfo, },
    program::invoke,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    info
};
use solana_bpf_ttp_oracle::{
    instruction::create_request,
    request::{ GetArgs, GetParams, JsonParseArgs, Request, Task },
};
use generic_array::GenericArray;
use arrayref::array_ref;

pub fn process_handle_response(
  _program_id: &Pubkey,
  _accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  let res_data = array_ref![instruction_data, 0, 4];
  // read the data sent back (le u32)
  let price = u32::from_le_bytes(*res_data);
  // Log the response
  info!(&format!("Oracle price response = {}", price));
  Ok(())
}

pub fn process_add_request(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();
  let oracle_program_account = next_account_info(accounts_iter)?;
  let oracle_account = next_account_info(accounts_iter)?;
  
  let request = create_example_request(program_id);
  
  let ix = create_request(
      oracle_program_account.key, 
      oracle_account.key, 
      request,
  )?;

  invoke(&ix, &[oracle_program_account.clone(), oracle_account.clone()])
}

fn create_example_request(_program_id: &Pubkey) -> Request {
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
    let uint_128_task = Task::Uint32;

    Request {
      tasks: [get_task, json_parse_task, uint_128_task],
      call_back_program: *_program_id,
      index: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{
      clock::Epoch,
      instruction::Instruction,
      program_error::ProgramError,
      program_pack::Pack,
      program_stubs,
    };
  use solana_bpf_ttp_oracle::{ 
    oracle_account::OracleAccount,
    processor::Processor,
    request::Request,
  };

    // test program id for ttp-oralce program
    const TTP_ORACLE_PROGRAM_ID: Pubkey = Pubkey::new_from_array([1u8; 32]);
    // test program id for the client program that consumes ttp-oracle
    const CLIENT_PROGRAM_ID: Pubkey = Pubkey::new_from_array([2u8; 32]);

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
              if *account_info.key != TTP_ORACLE_PROGRAM_ID {
                new_account_infos.push(account_info.clone())
              }
            }

            match instruction.program_id {
                TTP_ORACLE_PROGRAM_ID => invoke_oracle(&new_account_infos, &instruction.data),
                CLIENT_PROGRAM_ID => Ok(()),
                _ => Err(ProgramError::IncorrectProgramId)
            }
        }
    }

    fn invoke_oracle<'a>(account_infos: &[AccountInfo<'a>], input: &[u8]) -> ProgramResult {
      Processor::process(&TTP_ORACLE_PROGRAM_ID, account_infos, input)
    }

    fn setup_syscall_stubs() {
      use std::sync::Once;
      static ONCE: Once = Once::new();

      ONCE.call_once(|| {
          program_stubs::set_syscall_stubs(Box::new(TestSyscallStubs {}));
      });
    }

    #[test]
    fn program_invocation() {
      setup_syscall_stubs();
      let oracle_program_owner = Pubkey::default();
      let oracle_id = Pubkey::default();
      let mut lamports1 = 0;
      let mut lamports2 = 0;
      let mut oracle_data_buffer = vec![0; OracleAccount::LEN];
      
      let oracle_account = AccountInfo::new(&oracle_id, false, true, &mut lamports1, &mut oracle_data_buffer, &TTP_ORACLE_PROGRAM_ID, false, Epoch::default());
      let oracle_program_account = AccountInfo::new(&TTP_ORACLE_PROGRAM_ID, false, false, &mut lamports2, &mut [], &oracle_program_owner, true, Epoch::default());
      let accounts = vec![oracle_program_account, oracle_account];
      
      let ret = process_add_request(&CLIENT_PROGRAM_ID, &accounts, &[]);
      assert!(ret.is_ok());
      let request = create_example_request(&CLIENT_PROGRAM_ID);
      let mut expected_request = [0; Request::LEN];
      request.pack_into_slice(&mut expected_request);

      let oracle_data = OracleAccount::unpack(&oracle_data_buffer).unwrap();
      let ret_request = oracle_data.request_queue.requests[0].clone().unwrap();
      assert_eq!(ret_request, request);

      let mut request_buffer = [0u8; Request::LEN];
      Request::pack(ret_request, &mut request_buffer).unwrap();

      assert_eq!(request_buffer, expected_request);
    }
}