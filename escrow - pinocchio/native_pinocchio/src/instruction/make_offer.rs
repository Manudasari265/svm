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
    pub amount_mint_a: [u8; 8], //? amount of TokenA
    pub amount_mint_b: [u8; 8], //? amount of TokenB
    pub mint_a: //? maker offering this 'amount_mint_a' of 'mint_a' 
    pub mint_b //? maker requesting this 'amount_mint_b' of 'mint_b'
    pub bump: u8, //? user's canonical bump
}

impl DataLen for MakerOfferIxData {
    const LEN: usize = core::mem::size_of<MakerOfferIxData>();
}

use crate::{
    state::{DataLen, EscrowState, to_bytes, load_ix_data},
};

pub fn process_make_offer (
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [maker_acc, escrow_pda, maker_token_acc, vault_pda, mint_a, mint_b, _system_program, _token_program, _remaining @ ..] = accounts 
      else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let rent = Rent::get()?;

    let instruction_data = load_ix_data::<MakerOfferIxData>(instruction_data)?;

    let bump = [instruction_data.bump];

    Ok(())
}