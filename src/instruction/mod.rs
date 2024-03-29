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
            Nop | Not(_) | Ret | Cli => true,
            DB(_) => true, // TODO: Should this always be true?

            MovM2R(src, dest) => src.width() == Width::Word && dest.is_writable(),
            MovR2M(_, dest) => dest.width() == Width::Word,

            MovR2R(src, dest) |
            AddR2R(src, dest) | SubR2R(src, dest) | AndR2R(src, dest) | OrR2R(src, dest) | CmpR2R(src, dest)
                => src.width() == dest.width() && dest.is_writable(),

            MovC2R(value, dest) |
            AddC2R(value, dest) | SubC2R(value, dest) | AndC2R(value, dest) | OrC2R(value, dest) | CmpC2R(value, dest) 
                => value.width() == dest.width() && dest.is_writable(),

            CallC(value)
                => value.width() == Width::Word,

            Push(reg) | Pop(reg) |
            AJmp(reg) | Jmp(reg) | Jeq(reg) | Jneq(reg) | Jlt(reg) | Jgt(reg) | Jleq(reg) | Jgeq(reg) | Jo(reg) | Jno(reg) |
            CallR(reg) | Int(reg) | Sti(reg)
                => reg.width() == Width::Word,

            Shl(shift, reg)| Shr(shift, reg) | Shre(shift, reg)
                => match reg.width() {
                    Width::Byte => shift.value_byte(0) <= 8,
                    Width::Word => shift.value_byte(0) <= 16,
                }
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
            MovM2R(_, dest) => case!(dest, 0x05),
            MovR2M(src, _) => case!(src, 0x07),
            Push(_) => 0x09,
            Pop(_) => 0x0A,

            AddC2R(src, _) => case!(src, 0x0B),
            AddR2R(src, _) => case!(src, 0x0D),
            SubC2R(src, _) => case!(src, 0x0F),
            SubR2R(src, _) => case!(src, 0x11),
            Not(dest) => case!(dest, 0x13),
            AndC2R(src, _) => case!(src, 0x15),
            AndR2R(src, _) => case!(src, 0x17),
            OrC2R(src, _) => case!(src, 0x19),
            OrR2R(src, _) => case!(src, 0x1B),
            Shl(_, dest) => case!(dest, 0x1D),
            Shr(_, dest) => case!(dest, 0x1F),
            Shre(_, dest) => case!(dest, 0x21),
            CmpC2R(src, _) => case!(src, 0x23),
            CmpR2R(src, _) => case!(src, 0x25),

            AJmp(_) => 0x27,
            Jmp(_) => 0x28,
            Jeq(_) => 0x29,
            Jneq(_) => 0x2A,
            Jlt(_) => 0x2B,
            Jgt(_) => 0x2C,
            Jleq(_) => 0x2D,
            Jgeq(_) => 0x2E,
            Jo(_) => 0x2F,
            Jno(_) => 0x30,
            CallC(_) => 0x31,
            CallR(_) => 0x32,
            Ret => 0x33,

            Int(_) => 0x34,
            Sti(_) => 0x35,
            Cli => 0x36,
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u16 {
        use Instruction::*;
        match self {
            DB(_) => 1,

            Nop |
            MovR2R(_, _) | MovM2R(_, _) | MovR2M(_, _) | Push(_) | Pop(_) |
            AddR2R(_, _) | SubR2R(_, _) | Not(_) | AndR2R(_, _) | OrR2R(_, _) | Shl(_, _) | Shr(_, _) | Shre(_, _) | CmpR2R(_, _) |
            AJmp(_) | Jmp(_) | Jeq(_) | Jneq(_) | Jlt(_) | Jgt(_) | Jleq(_) | Jgeq(_) | Jo(_) | Jno(_) | CallR(_) | Ret |
            Int(_) | Sti(_) | Cli
                => 2,

            MovC2R(_, _) |
            AddC2R(_, _) | SubC2R(_, _) | AndC2R(_, _) | OrC2R(_, _) | CmpC2R(_, _) |
            CallC(_)
                => 4,
        }
    }
}

mod compile;

#[cfg(test)]
mod test;
