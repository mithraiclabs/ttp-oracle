use crate::{
  instruction::{ OracleInstruction },
  request::Request,
};
use solana_program::{
  account_info::{ next_account_info, AccountInfo },
  entrypoint::ProgramResult,
  program_error::ProgramError,
  program_pack::Pack,
  pubkey::Pubkey
};
use arrayref::array_mut_ref;

pub struct Processor {}
impl Processor {

  // process instructions
  pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = OracleInstruction::unpack(input)?;

    match instruction {
      OracleInstruction::CreateRequest { request } => Self::process_create_request(accounts, &request),
      _ => Err(ProgramError::InvalidArgument)
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
}

#[cfg(test)]
mod tests {
  use super::*;
  use generic_array::GenericArray;
  use solana_program::clock::Epoch;
  use crate::request::{
    GetArgs,
    GetParams,
    JsonParseArgs,
    Task
  };

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
    let uint_256_task = Task::SolUint256;
    let httpget_tag = [0 as u8; 4];
    let json_tag: [u8; 4] = [1, 0, 0, 0];
    let uint256_tag: [u8; 4] = [2, 0, 0, 0];

    return Request {
      tasks: [get_task, json_parse_task, uint_256_task],
      offset: 0
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
}