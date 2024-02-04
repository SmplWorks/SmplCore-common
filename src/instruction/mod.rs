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

    /// Push register to the stack
    Push(Register),

    /// Pop register from the stack
    Pop(Register),
    
    // Arithmetic
    /// Add value to register
    AddC2R(Value, Register),

    /// Add two registers
    AddR2R(Register, Register),

    /// Subtract value from register
    SubC2R(Value, Register),

    /// Subtract two registers
    SubR2R(Register, Register),

    /// Biwtise not a register
    Not(Register),

    /// Biwtise and a register with a value
    AndC2R(Value, Register),

    /// Biwtise and two registers
    AndR2R(Register, Register),

    /// Biwtise or a register with a value
    OrC2R(Value, Register),

    /// Biwtise or two registers
    OrR2R(Register, Register),

    /// Shift left a register
    Shl(Value, Register),

    /// Shift right a register
    Shr(Value, Register),

    /// Shift right a register (sign extended)
    Shre(Value, Register),

    /// Compare a register to a value
    CmpC2R(Value, Register),

    /// Compare two registers
    CmpR2R(Register, Register),

    // Jumps
    /// Absolute jump
    AJmp(Register),

    /// Relative jump
    Jmp(Register),

    /// Relative jump if equal
    Jeq(Register),

    /// Relative jump if not equal
    Jneq(Register),

    /// Relative jump if less than
    Jlt(Register),

    /// Relative jump if greater than
    Jgt(Register),

    /// Relative jump if less than or equal 
    Jleq(Register),

    /// Relative jump if greater than or equal 
    Jgeq(Register),

    /// Relative jump if overflow
    Jo(Register),

    /// Relative jump if not overflow
    Jno(Register),

    /// Push RIP and to the stack and relative jump
    CallC(Value),

    /// Push RIP and to the stack and relative jump
    CallR(Register),

    /// Pop RIP from the stack
    Ret,

    /// Send an interrupt with value of the register
    Int(Register),

    /// Enable interruptions and point handler to register
    Sti(Register),

    /// Disable interruptions
    Cli,
}

macro_rules! inst_constc {
    ($ident:ident, $variant:ident) => {
        pub fn $ident(value : Value) -> Result<Self> {
            let res = Self::$variant(value);
            res.is_valid().then_some(res).ok_or(Error::OperandWidthMismatch(res))
        }
    };
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
    inst_constr!(push, Push);
    inst_constr!(pop, Pop);

    inst_constc2r!(addc2r, AddC2R);
    inst_constr2r!(addr2r, AddR2R);
    inst_constc2r!(subc2r, SubC2R);
    inst_constr2r!(subr2r, SubR2R);
    inst_constr!(not, Not);
    inst_constc2r!(andc2r, AndC2R);
    inst_constr2r!(andr2r, AndR2R);
    inst_constc2r!(orc2r, OrC2R);
    inst_constr2r!(orr2r, OrR2R);
    inst_constc2r!(shl, Shl);
    inst_constc2r!(shr, Shr);
    inst_constc2r!(shre, Shre);
    inst_constc2r!(cmpc2r, CmpC2R);
    inst_constr2r!(cmpr2r, CmpR2R);

    inst_constr!(ajmp, AJmp);
    inst_constr!(jmp, Jmp);
    inst_constr!(jeq, Jeq);
    inst_constr!(jneq, Jneq);
    inst_constr!(jlt, Jlt);
    inst_constr!(jgt, Jgt);
    inst_constr!(jleq, Jleq);
    inst_constr!(jgeq, Jgeq);
    inst_constr!(jo, Jo);
    inst_constr!(jno, Jno);
    inst_constc!(callc, CallC);
    inst_constr!(callr, CallR);

    pub fn ret() -> Self {
        Self::Ret
    }

    inst_constr!(int, Int);
    inst_constr!(sti, Sti);

    pub fn cli() -> Self {
        Self::Cli
    }

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

            _ => todo!("{self:?}")
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

            _ => todo!("{self:?}")
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

            _ => todo!("{self:?}")
        }
    }
}

mod compile;

#[cfg(test)]
mod test;
