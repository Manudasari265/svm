pub mod make;
pub mod refund;
pub mod take;
pub mod delegate;

pub use make::*;
pub use refund::*;
pub use take::*;
pub mod delegate::*;

use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum MyProgramInstrution {
    Make,
    Take,
    Refund,
}

impl TryFrom<&u8> for MyProgramInstrution {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MyProgramInstrution::Make),
            1 => Ok(MyProgramInstrution::Take),
            2 => Ok(MyProgramInstrution::Refund),
            3 => Ok(MyProgramInstrution::Delegate),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
