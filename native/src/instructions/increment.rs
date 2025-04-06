use solana_program:: {
    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program::invoke,
    entrypoint::ProgramResult,
};
use crate::counter::Counter;

pub fn process_increment_counter (
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    initial_value: u64,
) -> ProgramResult {
    // let account = &accounts[0];
    // let mut counter = Counter::try_from_slice(&account.data.borrow())?;
    // counter.count += 1;
    // counter.serialize(&mut *account.data.borrow_mut())?;
    // Ok(())
    let counter_account = next_account_info(account_iter)?;
    let payer_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let account_iter = &mut accounts.iter();

    let account_space = 8;

    let rent = Rent::get()?;
    let rent_exempt = rent.minimum_balance(account_space);

    invoke (
        &system_instruction::create_account (
            payer_account.key,
            counter_account.key,
            rent_exempt,
            account_space as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}