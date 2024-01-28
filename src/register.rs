use crate::utils::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    Byte, Word, 
}

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

    /// General purpose register
    R(Width, u8),
}

impl Register {
    pub fn r(width : Width, number : u8) -> Result<Self> {
        if number > 11 {
            return Err(Error::InvalidRegisterNumber(number))
        }

        Ok(Self::R(width, number))
    }

    pub fn width(&self) -> Width {
        use Register::*;
        match self {
            RINFO | RIP | RINT | Flags => Width::Word,
            R(w, _) => *w,
        }
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
            R(width, number) => {
                let middle = match width {
                    Width::Byte => "b",
                    Width::Word => "",
                };
                write!(f, "R{}{}", middle, number)
            },
        }
    }
}
