use solana_program::program_error::ProgramError;

pub mod increment_counter;
pub mod decrement_counter;
pub mod initialize_counter;

pub use increment_counter::process_increment_counter;
pub use decrement_counter::process_decrement_counter;
pub use initialize_counter::process_counter_initialize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterInstruction {
    InitializeCounter { initial_value: u64, },
    IncrementCounter,
    DecrementCounter,
}

impl TryFrom<&u8> for CounterInstruction {
    type Error = ProgramError;

    fn try_from(discriminator: &u8) -> Result<Self, Self::Error> {
        match *discriminator {
            0 => Ok(CounterInstruction::InitializeCounter { initial_value: 0 }),
            1 => Ok(CounterInstruction::IncrementCounter),
            2 => Ok(CounterInstruction::DecrementCounter),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}