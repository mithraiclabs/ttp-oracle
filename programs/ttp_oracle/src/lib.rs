#![cfg(feature = "program")]

use solana_sdk::{
    account_info::AccountInfo,
    program_error::ProgramError,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};
// Load in our instruction enum
pub mod instruction;
pub mod request;

struct RequestBuffer {
  requests: [request::Request; 5]
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

  let oracle_instruction = instruction::OracleInstruction::unpack(instruction_data)?;
  match oracle_instruction {
    instruction::OracleInstruction::CreateRequest(request) => instruction::process_create_request_instruction(accounts, &request),
    _ => return Err(ProgramError::InvalidArgument),
  };
  Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

