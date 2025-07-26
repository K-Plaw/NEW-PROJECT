use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    declare_id
};

// Import the processor's process_instruction function and alias it.
use crate::processor::process_instruction as processor_process_instruction;

declare_id!("LdAWh3nDWt1repA9UVDeiMQifkFDoqSfoikNPe3zpnt");

// Use the aliased function as the program's entrypoint.
entrypoint!(processor_process_instruction);

#[cfg(not(feature = "no-entrypoint"))]
pub fn process_instruction(
    program_id: &Pubkey,       // Public key of the program account
    accounts: &[AccountInfo],  // Accounts passed to the program
    instruction_data: &[u8],   // Serialized instruction data
) -> ProgramResult {
    // Call the aliased function, avoiding recursion.
    processor_process_instruction(program_id, accounts, instruction_data)
}