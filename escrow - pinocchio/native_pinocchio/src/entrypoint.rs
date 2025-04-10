use crate::instruction::{self, EscrowInstruction};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
#[cfg(target_os = "solana")]
nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator_variant, instruction_data) = instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstruction::try_from(discriminator_variant) {
        EscrowInstruction::MakeOffer => {
            log!("Ix:0");
            instruction::process_make_offer(accounts, instruction_data)
        }
        EscrowInstruction::TakeOffer => {
            log!("Ix:1");
            instruction::process_take_offer(accounts, instruction_data)
        }
    }

    Ok(())
}

