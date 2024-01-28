use crate::{utils::{Error, Result}, Register, Value, Width};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // Misc
    /// No operation
    Nop,

    /// Pseudo instruction corresponding to a literal byte
    DB(u8),

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

    // Jumps
    Jmp(Register),
}

macro_rules! inst_const {
    ($ident:ident, $variant:ident) => {
        pub fn $ident(src : Register, dest : Register) -> Result<Self> {
            let res = Self::$variant(src, dest);
            res.is_valid().then_some(res).ok_or(Error::OperandWidthMismatch(res))
        }
    };
}

impl Instruction {
    pub fn nop() -> Self {
        Self::Nop
    }

    pub fn db(value : u8) -> Self {
        Self::DB(value)
    }

    pub fn movc2r(value : Value, dest : Register) -> Result<Self> {
        let res = Self::MovC2R(value, dest);
        res.is_valid().then_some(res).ok_or(Error::OperandWidthMismatch(res))
    }

    inst_const!(movr2r, MovR2R);
    inst_const!(movm2r, MovM2R);
    inst_const!(movr2m, MovR2M);
    inst_const!(add, Add);
    inst_const!(sub, Sub);

    pub fn jmp(reg : Register) -> Result<Self> {
        let res = Self::Jmp(reg);
        res.is_valid().then_some(res).ok_or(Error::OperandWidthMismatch(res))
    }

    pub fn is_valid(&self) -> bool {
        use Instruction::*;
        match self {
            Nop => true,
            DB(_) => true, // TODO: Should this always be true?

            MovC2R(value, dest) => value.width() == dest.width() && dest.is_writable(),
            MovM2R(src, dest) => src.width() == Width::Word && dest.width() == Width::Byte && dest.is_writable(),
            MovR2M(src, dest) => src.width() == Width::Byte && dest.width() == Width::Word && dest.is_writable(),

            MovR2R(src, dest) | Add(src, dest) | Sub(src, dest)
                => src.width() == dest.width() && dest.is_writable(),

            Jmp(reg) => reg.width() == Width::Word,
        }
    }

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
            DB(value) => *value,

            MovC2R(value, _) => case!(value, 0x01),
            MovR2R(src, _) => case!(src, 0x03),
            MovM2R(_, _) => 0x05,
            MovR2M(_, _) => 0x06,

            Add(src, _) => case!(src, 0x07),
            Sub(src, _) => case!(src, 0x09),

            Jmp(_) => 0x0B,
        }
    }

    pub fn len(&self) -> usize {
        use Instruction::*;
        match self {
            DB(_) => 1,

            Nop |
            MovR2R(_, _) | MovM2R(_, _) | MovR2M(_, _) |
            Add(_, _) | Sub(_, _) |
            Jmp(_)
                => 2,

            MovC2R(_, _) => 4,
        }
    }
}

mod compile;
pub use compile::*;

#[cfg(test)]
mod test;
