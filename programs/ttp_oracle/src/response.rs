use crate::{
  request::{ RequestIndex, REQUEST_INDEX_SIZE }
};
use solana_program::{
  program_error::ProgramError,
  program_pack::{ IsInitialized, Pack, Sealed },
};
use arrayref::{ array_ref, array_refs, array_mut_ref, mut_array_refs };

pub const RESPONSE_DATA_LEN: usize = 16;
pub const CALLBACK_DETERMINANT_LEN: usize = 1;

type ResponseData = [u8; 16];

#[derive(Debug, PartialEq)]
pub struct Response {
  pub data: ResponseData,
  pub request_queue_index: RequestIndex,
}

impl Response {
  const CALLBACK_DETERMINANT: u8 = 255;
}

impl Sealed for Response {}
impl IsInitialized for Response {
  fn is_initialized(&self) -> bool {
    true
  }
}
impl Pack for Response {
  const LEN: usize = RESPONSE_DATA_LEN + CALLBACK_DETERMINANT_LEN + 1;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, Response::LEN];
    let (_, data, request_queue_index) = array_refs![&src, CALLBACK_DETERMINANT_LEN, RESPONSE_DATA_LEN, REQUEST_INDEX_SIZE];
    Ok(Response {
      data: *data,
      request_queue_index: u8::from_le_bytes(*request_queue_index),
    })
  }
  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dest = array_mut_ref![dst, 0, Response::LEN];
    let (determinant, data, index) = mut_array_refs![dest, CALLBACK_DETERMINANT_LEN, RESPONSE_DATA_LEN, REQUEST_INDEX_SIZE];
    determinant.copy_from_slice(&u8::to_le_bytes(Response::CALLBACK_DETERMINANT));
    data.copy_from_slice(&self.data);
    index.copy_from_slice(&[self.request_queue_index]);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_pack_unpack_response() {
    let response_val: u128 = 15439;
    let response = Response {
      data: response_val.to_le_bytes(),
      request_queue_index: 1,
    };

    let &mut mut serialized_response = &mut [0 as u8; Response::LEN];
    Response::pack(response, &mut serialized_response).unwrap();
    let serialized_ref = array_ref![serialized_response, 0, Response::LEN]; 
    let (det, resp, index) = array_refs![serialized_ref, CALLBACK_DETERMINANT_LEN, RESPONSE_DATA_LEN, REQUEST_INDEX_SIZE];
    assert_eq!(resp, &response_val.to_le_bytes());
    assert_eq!(det, &Response::CALLBACK_DETERMINANT.to_le_bytes());
    assert_eq!(u8::from_le_bytes(*index), 1);

    let response = Response {
      data: response_val.to_le_bytes(),
      request_queue_index: 1,
    };

    let deserialized_response: Response = Response::unpack(&serialized_response).unwrap(); // Response::unpack_from_slice(&serialized_response).unwrap();

    assert_eq!(deserialized_response, response);
  }

}