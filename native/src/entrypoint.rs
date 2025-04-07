use crate::instruction::{self, CounterInstruction};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // divide the instruction data into 2 parts
    let (discriminator_variant, instruction_data) = instruction_data
              .split_first()
              .ok_or(ProgramError::InvalidInstructionData)?;
    
    match CounterInstruction::try_from(discriminator_variant)? {
        CounterInstruction::InitializeCounter{initial_value} => {
            msg!("Instruction Initialized");
            instruction::process_counter_initialize(program_id, accounts, instruction_data, initial_value)?;
        }
        CounterInstruction::IncrementCounter => {
            msg!("Increment Counter");
            instruction::process_increment_counter(program_id, accounts, instruction_data)?;
        }
        CounterInstruction::DecrementCounter => {
            msg!("Decrement Counter");
            instruction::process_decrement_counter(program_id, accounts, instruction_data)?;
        }
    }
    
    Ok(())
}