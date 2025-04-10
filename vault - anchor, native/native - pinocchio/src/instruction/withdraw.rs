use pinocchio::{
    account_info::AccountInfo, 
    instruction::{Seed, Signer}, 
    program_error::ProgramError, ProgramResult,
    pubkey::{self},
};

use pinocchio_system::instructions::Transfer;

use crate::
    state::{
        utils::load_ix_data, DataLen,
    };

use super::deposit::LAMPORTS_PER_SOL;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WithdrawIxData {
    pub amount: u64,
    pub bump: u8,
}

impl DataLen for WithdrawIxData {
    const LEN: usize = core::mem::size_of::<WithdrawIxData>();
}

pub fn process_withdraw (
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let [payer_acc, vault_acc, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let ix_data = load_ix_data::<WithdrawIxData>(data)?;

    /* (Or) simply use the below code, it also does the same */
    
    let pda_seeds = [
        "pinocchio_vault_pda".as_bytes(),
        payer_acc.key(),
        &[ix_data.bump]
    ];
    let expected_vault_pda = pubkey::create_program_address(
        &pda_seeds,
        &crate::ID
    )?;

    if expected_vault_pda != *vault_acc.key() {
        return Err(ProgramError::InvalidSeeds);
    }
    

    let binding = [ix_data.bump]; // this is the pda_bump_bytes 

    // now we need to convrt the raw seeds back to pinochio seed types for invoke_signed()
    let signer_seeds = [
        Seed::from("pinocchio_vault_pda".as_bytes()),
        Seed::from(payer_acc.key()),
        Seed::from(&binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    Transfer {
        from: vault_acc,
        to: payer_acc,
        lamports: vault_acc.lamports(),
    }
    .invoke_signed(&signers)?;

    Ok(())
}