pub fn process (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstruction::try_from_slice(instruction_data)?;

    match instruction {
        CounterInstruction::Logging => {
            logging();
        }

        CounterInstruction::Increment => {
            increment_counter(accounts, program_id)
        }
    }
}

fn logging() {
    msg!("Hello, Solana!");
}

fn increment_counter (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account = &accounts[0];
    let mut counter_data = Counter::try_from_slice(&account.data.borrow())?;
    counter_data.count += 1;
    counter_data.serialize(&mut *account.data.borrow_mut())?;
    Ok(())
}