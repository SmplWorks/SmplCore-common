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
    /// Add value to register
    AddC2R(Value, Register),

    /// Add two registers
    AddR2R(Register, Register),

    /// Subtract value from register
    SubC2R(Value, Register),

    /// Subtract two registers
    SubR2R(Register, Register),

    // Jumps
    /// Absolute jump
    AJmp(Register),

    /// Relative jump
    Jmp(Register),
}

macro_rules! inst_constr {
    ($ident:ident, $variant:ident) => {
        pub fn $ident(reg : Register) -> Result<Self> {
            let res = Self::$variant(reg);
            res.is_valid().then_some(res).ok_or(Error::OperandWidthMismatch(res))
        }
    };
}

macro_rules! inst_constc2r {
    ($ident:ident, $variant:ident) => {
        pub fn $ident(value : Value, dest : Register) -> Result<Self> {
            let res = Self::$variant(value, dest);
            res.is_valid().then_some(res).ok_or(Error::OperandWidthMismatch(res))
        }
    };
}

macro_rules! inst_constr2r {
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

    inst_constc2r!(movc2r, MovC2R);
    inst_constr2r!(movr2r, MovR2R);
    inst_constr2r!(movm2r, MovM2R);
    inst_constr2r!(movr2m, MovR2M);

    inst_constc2r!(addc2r, AddC2R);
    inst_constr2r!(addr2r, AddR2R);
    inst_constc2r!(subc2r, SubC2R);
    inst_constr2r!(subr2r, SubR2R);

    inst_constr!(ajmp, AJmp);
    inst_constr!(jmp, Jmp);

    pub fn is_valid(&self) -> bool {
        use Instruction::*;
        match self {
            Nop => true,
            DB(_) => true, // TODO: Should this always be true?

            MovM2R(src, dest) => src.width() == Width::Word && dest.width() == Width::Byte && dest.is_writable(),
            MovR2M(src, dest) => src.width() == Width::Byte && dest.width() == Width::Word && dest.is_writable(),

            MovR2R(src, dest) |
            AddR2R(src, dest) | SubR2R(src, dest)
                => src.width() == dest.width() && dest.is_writable(),

            MovC2R(value, dest) |
            AddC2R(value, dest) | SubC2R(value, dest)
                => value.width() == dest.width() && dest.is_writable(),

            AJmp(reg) | Jmp(reg)
                => reg.width() == Width::Word,
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

            AddC2R(src, _) => case!(src, 0x07),
            AddR2R(src, _) => case!(src, 0x09),
            SubC2R(src, _) => case!(src, 0x0B),
            SubR2R(src, _) => case!(src, 0x0D),

            AJmp(_) => 0x0F,
            Jmp(_) => 0x10,
        }
    }


    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u16 {
        use Instruction::*;
        match self {
            DB(_) => 1,

            Nop |
            MovR2R(_, _) | MovM2R(_, _) | MovR2M(_, _) |
            AddR2R(_, _) | SubR2R(_, _) |
            AJmp(_) | Jmp(_)
                => 2,

            MovC2R(_, _) |
            AddC2R(_, _) | SubC2R(_, _)
                => 4,
        }
    }
}

mod compile;

#[cfg(test)]
mod test;
