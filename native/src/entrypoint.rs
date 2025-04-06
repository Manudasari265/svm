use solana_program::{
    entrypoint,
    pubkey::Pubkey,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
};
use crate::processor::Processor;

entrypoint!(process_instruction);

