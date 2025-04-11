use pinocchio_token::state::TokenAccount;
use pinocchio_system::instruction::CreateAccount;
use pinocchio::{
    account_info::AccountInfo,
    sysvars::{rent::Rent, Sysvar},
    pubkey::Pubkey,
    ProgramResult,
};

#[repr(C)]
pub struct MakerOfferIxData {
    pub bump: u8, //? user's canonical bump
    pub amount_mint_a: [u8; 8], //? amount of TokenA
    pub amount_mint_b: [u8; 8], //? amount of TokenB
}

impl DataLen for MakerOfferIxData {
    const LEN: usize = core::mem::size_of<MakerOfferIxData>();
}

use crate::{
    state::{DataLen, EscrowState, to_bytes, load_ix_data},
};

pub fn process_make_offer_instruction (
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (maker_acc, mint_x, mint_y, maker_ata, vault_acc, escrow_pda, _system_program, _token_program, _remaining @..) = accounts
      else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    
    Ok(())
}