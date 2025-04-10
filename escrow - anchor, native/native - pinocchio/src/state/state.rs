use pinocchio::pubkey::Pubkey;
use bytemuck::{Pod, Zeroable};

use super::DataLen;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}

impl DataLen for Escrow {
    const LEN: usize = core::mem::size_of<Escrow>();
}

