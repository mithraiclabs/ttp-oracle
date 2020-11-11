use solana_program::{
  program_error::ProgramError,
  program_pack::{ IsInitialized, Pack, Sealed },
};
use arrayref::{ array_ref, array_refs, array_mut_ref, mut_array_refs };

const RESPONSE_DATA_LEN: usize = 16;
const CALLBACK_DETERMINANT_LEN: usize = 1;

type ResponseData = [u8; 16];

#[derive(Debug, PartialEq)]
pub struct Response {
  pub data: ResponseData
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
  const LEN: usize = RESPONSE_DATA_LEN + CALLBACK_DETERMINANT_LEN;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, Response::LEN];
    let (_, data) = array_refs![&src, CALLBACK_DETERMINANT_LEN, RESPONSE_DATA_LEN];
    Ok(Response {
      data: *data,
    })
  }
  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dest = array_mut_ref![dst, 0, Response::LEN];
    let (determinant, data) = mut_array_refs![dest, CALLBACK_DETERMINANT_LEN, RESPONSE_DATA_LEN];
    determinant.copy_from_slice(&u8::to_le_bytes(Response::CALLBACK_DETERMINANT));
    data.copy_from_slice(&self.data);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_pack_unpack_response() {
    let response_val: u128 = 15439;
    let response = Response {
      data: response_val.to_le_bytes()
    };

    let &mut mut serialized_response = &mut [0 as u8; Response::LEN];
    Response::pack(response, &mut serialized_response).unwrap();
    let serialized_ref = array_ref![serialized_response, 0, Response::LEN]; 
    let (det, resp) = array_refs![serialized_ref, CALLBACK_DETERMINANT_LEN, RESPONSE_DATA_LEN];
    assert_eq!(resp, &response_val.to_le_bytes());
    assert_eq!(det, &Response::CALLBACK_DETERMINANT.to_le_bytes());

    let response = Response {
      data: response_val.to_le_bytes()
    };

    let deserialized_response: Response = Response::unpack(&serialized_response).unwrap(); // Response::unpack_from_slice(&serialized_response).unwrap();

    assert_eq!(deserialized_response, response);
  }

}