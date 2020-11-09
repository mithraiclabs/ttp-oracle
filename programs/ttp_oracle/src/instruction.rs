use solana_program::{
  instruction::{ AccountMeta, Instruction },
  program_error::ProgramError,
  pubkey::Pubkey,
  program_pack::{ Pack, Sealed },
};
use arrayref::{ array_ref, array_refs, array_mut_ref, mut_array_refs };
use crate::request::{
  Request,
};

#[repr(C, u16)]
#[derive(Debug, PartialEq)]
pub enum OracleInstruction {
  /**
   * 0. [writable] the oracle to create request for
   */
  CreateRequest {
    // The request to be made by the oracle
    request: Request
  },
  
}
impl Sealed for OracleInstruction {}
impl Pack for OracleInstruction {
  const LEN: usize  = 142;

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, OracleInstruction::LEN];
    let (tag, serialized_request) = array_refs![src, 2, Request::LEN];
    return OracleInstruction::decode(*tag, *serialized_request);
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, OracleInstruction::LEN];
    let (
      tag_dst,
      data_dest,
    ) = mut_array_refs![dst, 2, Request::LEN];
    self.encode(tag_dst, data_dest)
  }
}

impl OracleInstruction {

  fn decode(tag: [u8; 2], data: [u8; Request::LEN]) -> Result<Self, ProgramError> {
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
    let instruction_data = array_ref![instruction_data, 0, OracleInstruction::LEN];

    let (tag, data) = array_refs![instruction_data, 2, Request::LEN];
    OracleInstruction::decode(*tag, *data)
  }
}
/// Generate the Instruction for CreateRequest.
/// Used for clients and cross program invocation
pub fn create_request(
  oracle_program_id: &Pubkey,
  oracle_id: &Pubkey,
  request: Request
) -> Result<Instruction, ProgramError> {
  let accounts = vec![AccountMeta::new(*oracle_id, false)];
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
  use crate::request::{
    GetArgs,
    GetParams,
    JsonParseArgs,
    Task
  };

  fn build_request() -> Request {
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

    return Request {
      tasks: [get_task, json_parse_task, uint_256_task],
      call_back_program: Pubkey::new_unique()
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
    let mut instruction_data = [0u8; OracleInstruction::LEN];
    create_req_instruction.pack_into_slice(&mut instruction_data);
    
    let res = OracleInstruction::unpack(&instruction_data).unwrap();
    assert_eq!(res, create_req_instruction);
  }

  #[test]
  fn test_create_request() {
    let oracle_program_id = Pubkey::default();
    let oracle_id = Pubkey::default();
    let call_back_program = Pubkey::new_unique();
    let mut request = build_request();
    request.call_back_program = call_back_program;
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

    let mut external_request = build_request();
    external_request.call_back_program = call_back_program;
    let ret = create_request(&oracle_program_id, &oracle_id, external_request).unwrap();
    assert_eq!(ret, instruction);
  }
}