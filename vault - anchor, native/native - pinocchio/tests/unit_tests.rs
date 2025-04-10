use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::{program, Mollusk};
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkey;
extern crate alloc;
use alloc::vec;

use native::instruction::DepositIxData;
use native::state::to_bytes;
use native::ID;

pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

pub const PAYER: Pubkey = pubkey!("7KuDrDJsLa2iKcUovWs7DFNYRdYJ12MyKyaJwnqmhSxy");

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/native");
    mollusk
}

#[test]
fn test_deposit() {
    //TODO: Steps to test the instruction functions👇
    //* bring the instance of mollusk
    //* system program and system account
    //* create the pda account
    //* initialize the accounts
    //* push the accounts into the instruction accounts ved!
    //* create the instruction data
    //* mention the first discriminator variant - Ix discriminator = 0
    //* serialize the instruction data
    //* create the instruction
    //* create the transaction accounts vec

    //? bring the instance of mollusk
    let mollusk = mollusk();

    //? system program and system account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    //?  create the pda account
    let (vault_pda, bump) = Pubkey::find_program_address(
        &["pinocchio_vault_pda".as_bytes(),
        &PAYER.to_bytes()],
        &PROGRAM_ID,
    );

    //? initialize the accounts
    let payer_account = Account::new(10 * LAMPORTS_PER_SOL, 0, &system_program);
    let vault_account = Account::new(0, 0, &system_program);

    //? push the accounts into the instruction accounts ved!
    let instruction_accounts = vec! [
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new(system_program, false),
    ];

    //? create the instruction data
    let instruction_data = DepositIxData {
        amount: 1, 
        bump,
    };

    //? mention the first discriminator variant - Ix discriminator = 0
    let mut ser_ix_data = vec![0];

    //? serialize the instruction data
    ser_ix_data.extend_from_slice(to_bytes(&instruction_data));

    //? create the instruction
    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID, 
        &ser_ix_data,
        instruction_accounts,
    );

    //? create the transaction accounts vec
    let txn_accounts = &vec! [
        (PAYER, payer_account.clone()),
        (vault_pda, vault_account.clone()),
        (system_program, system_account.clone()),
    ];

    let deposit_res = 
        mollusk.process_and_validate_instruction(&instruction, &txn_accounts, &[Check::success()]);

    assert!(deposit_res.program_result == ProgramResult::Success);
}

