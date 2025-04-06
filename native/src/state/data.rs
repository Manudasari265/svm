use Borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum CounterInstruction {
    InitializeCounter {
        initial_value: u64
    },
    IncrementCounter,
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // get the instruction variant from the first byte
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match variant {
            0 => {
                let initial_value = u64::from_le_byte(
                    rest.try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?
                );
                Ok(Self::InitializeCounter {
                    initial_value
                })
            }
            1 => Ok(Self::InitializeCounter),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}