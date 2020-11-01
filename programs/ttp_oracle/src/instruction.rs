#![cfg_attr(not(feature = "program"), allow(unused))]
use std::mem;
use std::cell::{RefCell, RefMut};
use solana_sdk::{
  account_info::{ next_account_info, AccountInfo },
  program_error::ProgramError,
  pubkey::Pubkey,
};
use serde::{ Serialize, Deserialize };
use arrayref::array_refs;
use crate::request::{
  GetArgs,
  GetParams,
  JsonParseArgs,
  Request,
  Task
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRequestData {
  request: Request,
}

// TODO maybe move this out since it's not coupled to instruction handling
pub type OracleResult<T = ()> = Result<T, ProgramError>;

impl CreateRequestData {
  /**
   * 
   * TODO serialize the Request from CreateRequestData and add it to the oracle data account
   * TODO allow handling more than 1 request. Current implementation simply overwrites the 
   *  entire account data buffer
   */
  pub fn process_instruction(&self, accounts: &[AccountInfo]) -> OracleResult {
    let accounts_iter = &mut accounts.iter();
    let oracle_account = next_account_info(accounts_iter)?;

    let mut data = oracle_account.try_borrow_mut_data()?;
    let serialized_request_res = bincode::serialize(&self);
    let mut serialized_request = match serialized_request_res {
      Err(_) => return Err(ProgramError::InvalidInstructionData),
      Ok(v) => v
    };

    // create padded serialized data for buffer size matches 
    let mut padded_serialized_request = [0 as u8; mem::size_of::<Request>()];
    padded_serialized_request[0..serialized_request.len()].copy_from_slice(serialized_request.as_slice());
    // overwrite account data
    data.copy_from_slice(&padded_serialized_request);

    Ok(())
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OracleInstruction {
  /**
   * 0. [writable] the oracle to create request for
   */
  CreateRequest(CreateRequestData)
}

impl OracleInstruction {

  pub fn unpack(instruction_data: &[u8]) -> Option<Self> {
    // Missing the u32 that determines the insutrction data
    if instruction_data.len() < 4 {
      return None;
    }

    let (&enum_bytes , data) = array_refs![instruction_data, 4; ..;];
    // extract the Instruction identifier
    let enum_idx = u32::from_le_bytes(enum_bytes);
  
    Some(match enum_idx {
      // TODO need to add methods to serialize and deserialize the request_data to Request
      0 =>  OracleInstruction::CreateRequest({
        let deserialized_request: Request = bincode::deserialize(&data).unwrap();
        CreateRequestData {
          request: deserialized_request
        }
      }),
      _ => return None,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use generic_array::GenericArray;
  use solana_sdk::clock::Epoch;
  use std::rc::Rc;
  use std::cell::{Ref};

  fn build_create_request() -> CreateRequestData {
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

    let req = Request {
      tasks: [get_task, json_parse_task, uint_256_task],
      offset: 0
    };

    return CreateRequestData {
      request: req,
    };
  }

  #[test]
  fn test_unpack_bad_data_length() {
    let instruction_data: &[u8; 3] = &[1, 3, 4];
    let res = OracleInstruction::unpack(instruction_data);
    assert!(res.is_none());
  }
  #[test]
  fn test_serde_create_request() {
    let key_arr = [0 as u8; 32];
    let create_req = build_create_request();

    let serialized_create_req = bincode::serialize(&create_req).unwrap();
    let deserialized_request: Request = bincode::deserialize(&serialized_create_req).unwrap();
    assert_eq!(deserialized_request, create_req.request);

    let deserialized_create_req: CreateRequestData = bincode::deserialize(&serialized_create_req).unwrap();
    assert_eq!(deserialized_create_req, create_req);
  }

  #[test]
  fn test_create_request_instruction() {
    let key_arr = [0 as u8; 32];
    let create_req = build_create_request();
    let create_req_instruction = OracleInstruction::CreateRequest(create_req);
    let instruction_data = bincode::serialize(&create_req_instruction).unwrap();
    
    let res = OracleInstruction::unpack(&instruction_data).unwrap();
    assert_eq!(res, create_req_instruction);
  }

  #[test]
  fn test_process_create_request() {
    let program_id = Pubkey::default();
    let oracle_id = Pubkey::default();
    let mut lamports = 0;
    // account data buffer with the size of a request
    let create_req = build_create_request();
    let mut data_buffer = vec![1; mem::size_of::<Request>()];
    let account = AccountInfo::new(&oracle_id, false, true, &mut lamports, &mut data_buffer, &program_id, false, Epoch::default());
    let accounts = vec![account];


    let ret = create_req.process_instruction(&accounts);
    assert!(ret.is_ok());
    
    let account_data = accounts[0].data.borrow();

    let deserialized_request: Request = bincode::deserialize(&account_data).unwrap();

    assert_eq!(deserialized_request, create_req.request);
  }
}