#![allow(unexpected_cfgs)]
use pinocchio::{
    account_info::AccountInfo,
    program_entrypoint, 
    no_allocator, nostd_panic_handler,
    pubkey::Pubkey, ProgramResult,
    program_error::ProgramError,
};
use pinocchio_log::log;
use crate::instruction::{self, VaultInstruction};

program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
#[cfg(target_os = "solana")]
nostd_panic_handler!();

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator_variant, instruction_data) = instruction_data
          .split_first()
          .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstruction::try_from(discriminator_variant)? {
        VaultInstruction::Deposit => {
            log!("Ix:0");
            instruction::process_deposit(accounts, instruction_data);
        }
        VaultInstruction::Withdraw => {
            log!("Ix:1");
            instruction::process_withdraw(accounts, instruction_data);
        }
    }

    Ok(())
}
