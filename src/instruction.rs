use crate::{Register, Value};

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
        use Instruction::*;

        macro_rules! case {
            ($var:ident, $low:literal) => {
                $var(Register::new_r(0, $low).unwrap(), Register::new_r(1, $low).unwrap())
            };
        }

        // All instructions
        let all = vec![
            Nop,

            MovC2R(Value::new_byte(1), Register::new_r(0, true).unwrap()),
            MovC2R(Value::new_byte(1), Register::new_r(0, false).unwrap()),
            case!(MovR2R, true),
            case!(MovR2R, false),
            case!(MovM2R, true),
            case!(MovM2R, false),
            case!(MovR2M, true),
            case!(MovR2M, false),

            case!(Add, true),
            case!(Add, false),
            case!(Sub, true),
            case!(Sub, false),
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
