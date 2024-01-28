use crate::utils::{Error, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// CPU information flags, 16-bits
    RINFO,

    /// Instruction pointer, 16-bits
    RIP, 

    /// Interrupt handler pointer, 16-bits
    RINT,

    /// Flags, always 16-bits
    Flags,

    /// General purpose register 16 or 8 bits depending on low
    R {
        /// Number, between 0 and 11
        number: u8,

        /// Is in low mode, only lower 8-bits
        low: bool
    },
}

impl Register {
    pub fn new_r(number : u8, low : bool) -> Result<Self> {
        if number > 11 {
            return Err(Error::InvalidRegisterNumber(number))
        }

        Ok(Self::R { number, low })
    }
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register::*;
        match self {
            RINFO => write!(f, "RINFO"),
            RIP => write!(f, "RIP"),
            RINT => write!(f, "RINT"),
            Flags => write!(f, "Flags"),
            R { number, low } => write!(f, "R{}{}", if *low { "b" } else { "" }, number),
        }
    }
}
