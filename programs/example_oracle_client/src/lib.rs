use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

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
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
  processor::process_add_request(_program_id, accounts, _instruction_data)
}
