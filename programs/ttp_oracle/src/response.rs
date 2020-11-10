use solana_program::{
  instruction::{ AccountMeta, Instruction },
  program_error::ProgramError,
  pubkey::Pubkey,
  program_pack::{ Pack, Sealed },
};
use arrayref::{ array_ref, array_refs, array_mut_ref, mut_array_refs };

#[derive(Debug, PartialEq)]
pub struct Response {
  pub data: [u8; 16]
}

impl Sealed for Response {}
impl Pack for Response {
  const LEN: usize = 16;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, Response::LEN];
    Ok(Response {
      data: *src
    })
  }
   fn pack_into_slice(&self, dst: &mut [u8]) {
      dst.copy_from_slice(&self.data);
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
    response.pack_into_slice(&mut serialized_response);
    assert_eq!(serialized_response[0..16], response_val.to_le_bytes());

    let deserialized_response: Response = Response::unpack_from_slice(&serialized_response).unwrap();

    assert_eq!(deserialized_response, response);
  }

}