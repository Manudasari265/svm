use pinocchio_token::state::TokenAccount;
use pinocchio_system::instruction::CreateAccount;
use pinocchio::{
    account_info::AccountInfo,
    sysvars::{rent::Rent, Sysvar},
    pubkey::Pubkey,
    ProgramResult,
};

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
    ///? Steps to process the instruction:
    /// extract the required accounts from the `accounts: &[AccountInfo]`` array
    /// mention fail safe checks for the accounts and data length
    /// check if the maker account is a signer
    /// extract the bump seeds from the instruction data and bind seeds for the PDA validation
    /// create the PDA for the escrow account and validate the PDA
    /// for the vault account, we can either create a new one or use it from the instruction data
    /// verify if vault is owned by the escrow program
    /// check if the escrow account is initialized
    /// check if the vault account is initialized
    /// Create the account for escrow with the required space 
    /// initialize the escrow account with the required data
    /// transfer the tokens from the maker to the vaultq
    
    //? extract the required accounts from the `accounts: &[AccountInfo]`` array
    let (maker_acc, mint_x, mint_y, maker_ata, vault, escrow_acc, _system_program, _token_program, _remaining @..) = accounts
      else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //? check if the maker account is a signer
    if !maker_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    //? mention fail safe checks for the accounts and data length
    if instruction_data.len() != MakerOfferIxData::LEN {
        return Err(ProgramError::InvalidInstructionData);
    }

    //? extract the bump seeds from the instruction data and bind seeds for the PDA validation
    let bump = unsafe {
        *(instruction_data.as_ptr() as *const u8)
    }.to_le_bytes();
    let seed = [(b"escrow"), maker_acc.key().as_slice(), bump.as_ref()];
    let seeds = &seed[..];

    
    Ok(())
}