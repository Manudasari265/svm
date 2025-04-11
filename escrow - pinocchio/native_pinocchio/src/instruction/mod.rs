use pinocchio::program_error::ProgramError;

#[repr(C)]
pub enum EscrowInstruction {
    MakeOffer = 0,
    TakeOffer = 1,
    Refund = 2,
}

impl TryFrom<&u8> for EscrowInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(EscrowInstruction::MakeOffer),
            1 => Ok(EscrowInstruction::TakeOffer),
            2 => Ok(EscrowInstruction::Refund),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

