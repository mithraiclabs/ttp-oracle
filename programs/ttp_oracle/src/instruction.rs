#![cfg_attr(not(feature = "program"), allow(unused))]
use std::mem;
use solana_sdk::{
  account_info::{ next_account_info, AccountInfo },
  instruction::{ AccountMeta, Instruction },
  program_error::ProgramError,
  pubkey::Pubkey,
  program_pack::{ Pack, Sealed },
};
use arrayref::{ array_ref, array_refs, array_mut_ref, mut_array_refs };
use crate::request::{
  GetArgs,
  GetParams,
  JsonParseArgs,
  Request,
  Task
};

// TODO maybe move this out since it's not coupled to instruction handling
pub type OracleResult<T = ()> = Result<T, ProgramError>;

/**
 * 
 * TODO serialize the Request from CreateRequestData and add it to the oracle data account
 * TODO allow handling more than 1 request. Current implementation simply overwrites the 
 *  entire account data buffer
 */
pub fn process_create_request_instruction(accounts: &[AccountInfo], request: &Request) -> OracleResult {
  let accounts_iter = &mut accounts.iter();
  let oracle_account = next_account_info(accounts_iter)?;

  let mut data = oracle_account.try_borrow_mut_data()?;
  let mut serialized_request = [0u8; Request::LEN];
  request.pack_into_slice(&mut serialized_request);

  // create padded serialized data for buffer size matches 
  let mut padded_serialized_request = [0 as u8; mem::size_of::<Request>()];
  padded_serialized_request[0..serialized_request.len()].copy_from_slice(&serialized_request);
  // overwrite account data
  data.copy_from_slice(&padded_serialized_request);

  Ok(())
}

#[repr(C, u16)]
#[derive(Debug, PartialEq)]
pub enum OracleInstruction {
  /**
   * 0. [writable] the oracle to create request for
   */
  CreateRequest {
    // The request to be made by the oracle
    request: Request
  }
}
impl Sealed for OracleInstruction {}
impl Pack for OracleInstruction {
  const LEN: usize  = 114;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, 114];
    let (tag, serialized_request) = array_refs![src, 2, 112];
    return OracleInstruction::decode(*tag, *serialized_request);
  }
   fn pack_into_slice(&self, dst: &mut [u8]) {
     let dst = array_mut_ref![dst, 0, 114];
     let (
        tag_dst,
        data_dest,
      ) = mut_array_refs![dst, 2, 112];
      self.encode(tag_dst, data_dest)
    }
}

impl OracleInstruction {

  fn decode(tag: [u8; 2], data: [u8; 112]) -> Result<Self, ProgramError> {
    match u16::from_le_bytes(tag) {
      0 => Ok(OracleInstruction::CreateRequest {
        request: Request::unpack_from_slice(&data)?
      }),
      _ => Err(ProgramError::InvalidInstructionData),
    }
  }

  fn encode(&self, kind: &mut [u8], data:&mut [u8])  {
    match self {
      OracleInstruction::CreateRequest { request } => {
        let tag: u16 = 0;
        kind.copy_from_slice(&tag.to_le_bytes()[0..2]);
        request.pack_into_slice(data);
      },
      // TODO propogate error here?
    }
  }

  pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
    // Missing the u32 that determines the insutrction data
    if instruction_data.len() < 2 {
      return Err(ProgramError::InvalidInstructionData);
    }
    let instruction_data = array_ref![instruction_data, 0, 114];

    let (tag, data) = array_refs![instruction_data, 2, 112];
    OracleInstruction::decode(*tag, *data)
  }
}

pub fn create_request(
  oracle_program_id: &Pubkey,
  oracle_id: &Pubkey,
  request: Request
) -> Result<Instruction, ProgramError> {
  let mut accounts = vec![AccountMeta::new(*oracle_id, false)];
  let mut data  = [0u8; OracleInstruction::LEN];
  OracleInstruction::CreateRequest { request }.pack_into_slice(&mut data);
  let data = data.to_vec();
  Ok(Instruction {
    program_id: *oracle_program_id,
    accounts,
    data,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use generic_array::GenericArray;
  use solana_sdk::clock::Epoch;
  use std::rc::Rc;
  use std::cell::{Ref};

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
  fn test_unpack_bad_data_length() {
    let instruction_data: &[u8; 1] = &[1];
    let res = OracleInstruction::unpack(instruction_data);
    assert_eq!(res, Err(ProgramError::InvalidInstructionData));
  }

  #[test]
  fn test_create_request_instruction() {
    let request = build_request();
    let create_req_instruction = OracleInstruction::CreateRequest {request };
    let mut instruction_data = [0u8; OracleInstruction::LEN + 4];
    create_req_instruction.pack_into_slice(&mut instruction_data);
    
    let res = OracleInstruction::unpack(&instruction_data).unwrap();
    assert_eq!(res, create_req_instruction);
  }

  #[test]
  fn test_process_create_request() {
    let program_id = Pubkey::default();
    let oracle_id = Pubkey::default();
    let mut lamports = 0;
    // account data buffer with the size of a request
    let request = build_request();
    let mut data_buffer = vec![1; mem::size_of::<Request>()];
    let account = AccountInfo::new(&oracle_id, false, true, &mut lamports, &mut data_buffer, &program_id, false, Epoch::default());
    let accounts = vec![account];


    let ret = process_create_request_instruction(&accounts, &request);
    assert!(ret.is_ok());
    
    let account_data = accounts[0].data.borrow();

    let deserialized_request: Request = Request::unpack_from_slice(&account_data).unwrap();

    assert_eq!(deserialized_request, request);
  }

  #[test]
  fn test_create_request() {
    let oracle_program_id = Pubkey::default();
    let oracle_id = Pubkey::default();
    let request = build_request();
    let account = AccountMeta::new(oracle_id, false);
    let accounts = vec![account];
    let mut data  = [0u8; OracleInstruction::LEN];
    OracleInstruction::CreateRequest { request }.pack_into_slice(&mut data);
    let data = data.to_vec();
    let instruction = Instruction {
      program_id: oracle_program_id,
      accounts,
      data
    };

    let external_request = build_request();
    let ret = create_request(&oracle_program_id, &oracle_id, external_request).unwrap();
    assert_eq!(ret, instruction);
  }
}