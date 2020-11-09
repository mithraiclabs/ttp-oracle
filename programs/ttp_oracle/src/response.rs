use solana_program::{
  instruction::{ AccountMeta, Instruction },
  program_error::ProgramError,
  pubkey::Pubkey,
  program_pack::{ Pack, Sealed },
};
use arrayref::{ array_ref, array_refs, array_mut_ref, mut_array_refs };

#[derive(Debug, PartialEq)]
pub struct Response {
  pub call_back_program: Pubkey,
  pub data: [u8; 16]
}

impl Sealed for Response {}
impl Pack for Response {
  const LEN: usize = 48;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, 48];
    let (call_back_program, data) = array_refs![src, 32, 16];
    Ok(Response {
      call_back_program: Pubkey::new_from_array(*call_back_program),
      data: *data
    })
  }
   fn pack_into_slice(&self, dst: &mut [u8]) {
      let dst = array_mut_ref![dst, 0, Response::LEN];
      let (call_back_program, data) =
        mut_array_refs![dst, 32, 16];
      call_back_program.copy_from_slice(&self.call_back_program.to_bytes());
      data.copy_from_slice(&self.data);
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_pack_unpack_response() {
    let public_key = Pubkey::new_unique();
    let response_val: u128 = 15439;
    let response = Response {
      call_back_program: public_key,
      data: response_val.to_le_bytes()
    };

    let &mut mut serialized_response = &mut [0 as u8; Response::LEN];
    response.pack_into_slice(&mut serialized_response);
    assert_eq!(serialized_response[0..32], public_key.to_bytes());
    assert_eq!(serialized_response[32..48], response_val.to_le_bytes());

    let deserialized_response: Response = Response::unpack_from_slice(&serialized_response).unwrap();

    assert_eq!(deserialized_response, response);
  }

}