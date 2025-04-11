use pinocchio::{
    account_info::AccountInfo,
    pubkey::Pubkey,
};
use pinocchio::program_error::ProgramError;


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EscrowState {   /// DATA BYTES
    pub maker: Pubkey,     /// 32 bytes
    pub mint_x: Pubkey,    /// 32 bytes
    pub mint_y: Pubkey,    /// 32 bytes
    pub amount: u64,       /// 8 bytes
    pub bump: u8,          /// 1 byte
    pub seed: u64,         /// 8 bytes(optionl here)
}

impl DataLen for EscrowState {
    //* LEN: usize = 32 + 32 + 32 + 8 + 1 + 8
    const LEN: usize = core::mem::size_of<EscrowState>();
}

impl EscrowState {
    /// load_mut_raw_unchecked
    /// load_mut_safe_check
    pub fn load_mut_raw_unchecked
}




