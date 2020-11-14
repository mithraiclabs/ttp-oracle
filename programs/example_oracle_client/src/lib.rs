use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{ array_ref };
use solana_bpf_ttp_oracle::processor::CALLBACK_DETERMINANT;

pub mod processor;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

/**
 * Takes 2 accounts
 * 
 * 0. Oracle Program - program Id of the oracle program
 * 1. [writable] Oracle - Account for the oracle to make the request
 */
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
  let tag = array_ref![instruction_data, 0, 1];
  match u8::from_le_bytes(*tag) {
    0 => {
      processor::process_add_request(program_id, accounts, &[])
    },
    CALLBACK_DETERMINANT => {
      processor::process_handle_response(program_id, accounts, &instruction_data[1..])
    },
    _ => Err(ProgramError::InvalidInstructionData),
  }
}
