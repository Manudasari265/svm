use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey, 
};

use crate::Counter;

pub fn process_increment_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [payer_account, counter_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };

    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !counter_account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // mutably borrow the account data
    let counter_data = &mut counter_account.data.borrow_mut();

    // deserialize the counter data into Counter struct
    let mut deserialize_counter = Counter::try_from_slice(&counter_data)?;

    deserialize_counter.count = deserialize_counter
         .count
         .checked_add(1)
         .ok_or(ProgramError::InvalidAccountData)?;

    deserialize_counter.serialize(&mut &mut counter_data[..])?;

    // let mut increment_counter = &mut counter_account.data.borrow_mut()[..];
    // let mut counter_data = increment_counter.get_mut(0..8).ok_or(ProgramError::InvalidAccountData)?;
    // let mut counter_value = u64::from_le_bytes(counter_data.try_into().map_err(|_| ProgramError::InvalidAccountData)?);
    // counter_value += 1;
    // counter_data.copy_from_slice(&counter_value.to_le_bytes());

    // // Serialize the updated counter value back to the account data
    // let mut account_data = &mut counter_account.data.borrow_mut()[..];
    // let mut counter_data = Counter {
    //     counter: counter_value,
    // };
    // counter_data.serialize(&mut account_data)?;
    

    Ok(())
}