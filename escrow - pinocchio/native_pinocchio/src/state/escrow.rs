use pinocchio::{
    account_info::AccountInfo,
    pubkey::Pubkey,
};
use pinocchio::program_error::ProgramError;


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EscrowState {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub maker: Pubkey,
    pub receive: u64,
    pub bump: u8,
}

impl DataLen for EscrowState {
    const LEN: usize = core::mem::size_of<EscrowInstruction>();
}




