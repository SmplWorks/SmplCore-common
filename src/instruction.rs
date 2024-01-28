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

    pub fn movr2r(src : Register, dest : Register) -> Result<Self> {
        let res = Self::MovR2R(src, dest);
        if src.width() == dest.width() {
            Ok(res)
        } else {
            Err(Error::OperandWidthMismatch(res))
        }
    }

    pub fn movm2r(src : Register, dest : Register) -> Result<Self> {
        let res = Self::MovM2R(src, dest);
        if src.width() == Width::Word && dest.width() == Width::Byte {
            Ok(res)
        } else {
            Err(Error::OperandWidthMismatch(res))
        }
    }

    pub fn movr2m(src : Register, dest : Register) -> Result<Self> {
        let res = Self::MovM2R(src, dest);
        if src.width() == Width::Byte && dest.width() == Width::Word {
            Ok(res)
        } else {
            Err(Error::OperandWidthMismatch(res))
        }
    }

    pub fn add(src : Register, dest : Register) -> Result<Self> {
        let res = Self::Add(src, dest);
        if src.width() == dest.width() {
            Ok(res)
        } else {
            Err(Error::OperandWidthMismatch(res))
        }
    }

    pub fn sub(src : Register, dest : Register) -> Result<Self> {
        let res = Self::Sub(src, dest);
        if src.width() == dest.width() {
            Ok(res)
        } else {
            Err(Error::OperandWidthMismatch(res))
        }
    }

    pub fn opcode(&self) -> u8 {
        use Instruction::*;
        match self {
            Nop => 0x00,

            MovC2R(value, dest) => 0x01,
            MovR2R(src, dest) => 0x02,
            MovR2M(src, dest) => 0x03,
            MovM2R(src, dest) => 0x04,

            Add(src, dest) => 0x05,
            Sub(src, dest) => 0x06,
        }
    }
}

#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn all_different_opcodes() {
        use crate::Width;

        macro_rules! case {
            ($var:ident, $width:ident) => {
                Instruction::$var(Register::r(Width::$width, 1).unwrap(), Register::r(Width::$width, 1).unwrap()).unwrap()
            };
        }

        // All instructions
        let all = vec![
            Instruction::nop(),

            Instruction::movc2r(Value::byte(1), Register::r(Width::Byte, 0).unwrap()).unwrap(),
            Instruction::movc2r(Value::word(1), Register::r(Width::Word, 0).unwrap()).unwrap(),
            case!(movr2r, Byte),
            case!(movr2r, Word),
            case!(movm2r, Byte),
            case!(movm2r, Word),
            case!(movr2m, Byte),
            case!(movr2m, Word),

            case!(add, Byte),
            case!(add, Word),
            case!(sub, Byte),
            case!(sub, Word),
        ];

        for inst0 in all.iter() {
            for inst1 in all.iter() {
                if inst0 != inst1 {
                    assert_ne!(inst0.opcode(), inst1.opcode(), "{:?} {:?}", inst0, inst1);
                }
            }
        }
    }
}
