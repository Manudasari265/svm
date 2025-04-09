use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_error::ProgramError, pubkey::Pubkey, 
    system_instruction::create_account, sysvar::{rent::Rent, Sysvar}
};

use crate::Counter;

pub fn process_counter_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_value: u64,
) -> ProgramResult {
    let [
        payer_account, 
        counter_account, 
        _sysvar_rent_program, 
        system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !counter_account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized)
    }

    let counter_account_space = 8;

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(counter_account_space);

    invoke(
        &create_account(
            payer_account.key, 
            counter_account.key, 
            required_lamports, 
            counter_account_space as u64, 
            program_id,
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    let counter_data = Counter {
        count: initial_value,
    };

    let mut account_data = &mut counter_account.data.borrow_mut()[..];
    counter_data.serialize(&mut account_data)?;

    Ok(())
}