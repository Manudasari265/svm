use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::{
    error::MyProgramError,
    state::{DataLen, Escrow},
};

pub const DELEGATION_ACCOUNT: Pubkey = pubkey!("");

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct DelegateAccountArgs {
    pub commit_frequency_ms: u32,
    pub seeds: Vec<Vec<u8>>,
    pub validator: Option<Pubkey>,
}

impl Default for DelegateAccountArgs {
    fn default() -> Self {
        DelegateAccountArgs {
            commit_frequency_ms: u32::MAX,
            seeds: vec![],
            validator: None,
        }
    }
}

pub fn process_delegate (
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let [maker, pda_acc, magic_acc, buffer_acc, delegation_acc, _sytem_programn] = accounts 
      else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // get buffer seeds
    let buffer_seeds: &[&[u8]] = &[b"buffer", accounts.pda_acc.key().as_ref()];
    let escrow_seeds = &[b"escrow", maker.key().as_ref()];

    // find pdas
    let (_, delegate_account_bump) = pubkey::find_program_address(
        escrow_seeds,
        &crate::ID
    );
    let (_, buffer_pda, bump) = pubkey::find_program_address(
        buffer_seeds,
        &crate::ID
    );

    // get signer seeds
    let bump = [delegate_account_bump];
    let seed = [
        Seed::from(b"escrow"),
        Seed::from(maker.key().as_ref()),
        Seed::from(&bump),
    ];
    let seed_b = [
        Seed::from(b"buffer"),
        Seed::from(accounts.pda_acc.key().as_ref()),
        Seed::from(&bump),
    ];
    // let bump_signer = Signer::from(&seed);

    let bump_signer_seeds = Signer::from(&seed_b);

    pinocchio_system::instruction::CreateAccount {
        from: maker,
        to: buffer_acc,
        lamports: Rent::get()?.minimum_balance(Escrow::LEN),
        space: Escrow::LEN as u64, // PDA acc length
        owner: &crate::ID,
    }
    .invoke_signed(&[buffer_signer_seeds].clone())?;

    // copy the data to the buffer PDA
    let mut buffer_data = buffer_acc.try_borrow_mut_data()?;
    lut new_data = pda_acc.try_borrow_mut_data()?.to_vec().clone();
    (*buffer_data).copy_from_slice(&new_data);
    drop(buffer_data);

    // acc needs to be closed to be delegated
    // zeroed lamports
    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *pda_acc.borrow_lamports_unchecked();
        *pda_acc.borrow_mut_lamports_unchecked() = 0;
    };

    // empty data
    pda_acc.realloc(0, false).unwrap();
    // send to system program
    unsafe {
        pda_acc.assign(&DELEGATION_ACCOUNT);
    };

    // create the delegation program
    pinocchio_system::instruction::CreateAccount {
        from: maker,
        to: pda_acc,
        lamports: Rent::get()?.minimum_balance(Escrow:LEN),
        space: Escrow::LEN as u64,
        owner: &crate::ID,
    }

}