use solana_program::{
    entrypoint::ProgramResult,
    account_info::AccountInfo, pubkey::Pubkey,
};

pub fn process_decrement_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    Ok(())
}