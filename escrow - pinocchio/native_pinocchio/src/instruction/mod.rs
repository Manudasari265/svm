use pinocchio::program_error::ProgramError;

#[repr(C)]
pub enum EscrowInstruction {
    MakeOffer,
    TakeOffer,
}

impl TryFrom<&u8> for EscrowInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(EscrowInstruction::MakeOffer),
            1 => Ok(EscrowInstruction::TakeOffer),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

