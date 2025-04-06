use solana_program::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    program_error::ProgramError
};
use borsh::BorshDeserialize;


#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum CounterInstruction {
    Logging,
    Increment,
    Update,
}

pub fn process_instruction (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // deserialize the instruction data
    let instruction_data = CounterInstruction::try_from_slice(instruction_data)?;

    // match the instruction data to the corresponding instruction
    match instruction_data {
        CounterInstruction::Increment => {
            process_increment_counter(accounts, program_id);
        }
    }
    Ok(())
}
