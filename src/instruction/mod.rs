use crate::{utils::{Error, Result}, Register, Value, Width};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // Misc
    /// No operation
    Nop,

    // Memory manipulation
    /// Move constant to register
    MovC2R(Value, Register), 

    /// Move register to register
    MovR2R(Register, Register),

    /// Move from memory to register
    MovM2R(Register, Register),

    /// Move from register to memory
    MovR2M(Register, Register),
    
    // Arithmetic
    /// Add two registers
    Add(Register, Register),

    /// Subtract two registers
    Sub(Register, Register),
}

macro_rules! inst_const {
    ($ident:ident, $variant:ident) => {
        pub fn $ident(src : Register, dest : Register) -> Result<Self> {
            let res = Self::$variant(src, dest);
            if src.width() == dest.width() {
                Ok(res)
            } else {
                Err(Error::OperandWidthMismatch(res))
            }
        }
    };
}

impl Instruction {
    pub fn nop() -> Self {
        Self::Nop
    }

    pub fn movc2r(value : Value, dest : Register) -> Result<Self> {
        let res = Self::MovC2R(value, dest);
        if value.width() == dest.width() {
            Ok(res)
        } else {
            Err(Error::OperandWidthMismatch(res))
        }
    }

    inst_const!(movr2r, MovR2R);

    pub fn movm2r(src : Register, dest : Register) -> Result<Self> {
        let res = Self::MovM2R(src, dest);
        if src.width() == Width::Word && dest.width() == Width::Byte {
            Ok(res)
        } else {
            Err(Error::OperandWidthMismatch(res))
        }
    }

    pub fn movr2m(src : Register, dest : Register) -> Result<Self> {
        let res = Self::MovR2M(src, dest);
        if src.width() == Width::Byte && dest.width() == Width::Word {
            Ok(res)
        } else {
            Err(Error::OperandWidthMismatch(res))
        }
    }

    inst_const!(add, Add);
    inst_const!(sub, Sub);

    pub fn opcode(&self) -> u8 {
        use Instruction::*;
        use Width::*;

        macro_rules! case {
            ($ident:ident, $base:literal) => {
                match $ident.width() {
                    Byte => $base,
                    Word => $base + 1,
                }
            };
        }

        match self {
            Nop => 0x00,

            MovC2R(value, _) => case!(value, 0x01),
            MovR2R(src, _) => case!(src, 0x03),
            MovM2R(_, _) => 0x05,
            MovR2M(_, _) => 0x06,

            Add(src, _) => case!(src, 0x07),
            Sub(src, _) => case!(src, 0x09),
        }
    }
}

mod compile;

#[cfg(test)]
mod test;
