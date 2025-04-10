use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::{program, Mollusk};
use native::instruction::DepositIxtData;
use native::state::to_bytes;
use native::ID;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("CehwAR3MrvCGzED4P1sdGMuckKqP9MambEQvsueYdy9x");

pub fn mollusk() -> Mollusk {
    Mollusk::new(&PROGRAM, "target/deploy/native")
}

#[test]
fn test_deposit() {
    let mollusk = mollusk();

    let (system_program, system_account) = program::keyed_account_for_system_program();

    let (vault_pda, bump) = Pubkey::find_program_address(
        &["pinocchio_vault_pda".as_bytes(), &PAYER.to_bytes()],
        &PROGRAM,
    );

    let payer_acc = Account::new(10 * LAMPORTS_PER_SOL, 0, &system_program);
    let vault_acc = Account::new(0, 0, &system_program);

    let ix_account = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new(system_program, false),
    ];

    let ix_data = DepositIxtData { amount: 1, bump };

    let mut ser_ix_data = vec![0];

    ser_ix_data.extend_from_slice(to_bytes(&ix_data));

    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_account);

    let tx_accounts = &vec![
        (PAYER, payer_acc.clone()),
        (vault_pda, vault_acc.clone()),
        (system_program, system_account.clone()),
    ];

    let init_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(init_res.program_result == ProgramResult::Success);
}

#[test]
fn test_withdraw() {
    let mollusk = mollusk();

    let (system_program, system_account) = program::keyed_account_for_system_program();

    let (vault_pda, bump) = Pubkey::find_program_address(
        &["pinocchio_vault_pda".as_bytes(), &PAYER.to_bytes()],
        &PROGRAM,
    );

    let payer_acc = Account::new(10 * LAMPORTS_PER_SOL, 0, &system_program);
    let vault_acc = Account::new(0, 0, &system_program);

    let ix_account = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new(system_program, false),
    ];

    let ix_data = DepositIxtData { amount: 1, bump };

    let mut ser_ix_data = vec![1];

    ser_ix_data.extend_from_slice(to_bytes(&ix_data));

    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_account);

    let tx_accounts = &vec![
        (PAYER, payer_acc.clone()),
        (vault_pda, vault_acc.clone()),
        (system_program, system_account.clone()),
    ];

    let init_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(init_res.program_result == ProgramResult::Success);
}

