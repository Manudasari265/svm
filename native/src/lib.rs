#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod error;
pub mod instruction;
pub mod state;

pub use instruction::*;
pub use state::*;


solana_program::declare_id!("CXSvTq9VLfDSNPQGEAF8LF5YbpxZbMBbn5RzsvnGiQxw");