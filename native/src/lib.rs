#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod error;
pub mod instruction;
pub mod state;

pub use instruction::*;
pub use state::*;


solana_program::declare_id!("E5kv2j41SfsrZyCeEohk8SQ3i71Yzgiv32ey8ekeL5mQ");