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

#[cfg(test)]
mod test { 
    use super::*;

    macro_rules! check_widths_ok {
        ($ident:ident, $r0:ident, $r1:ident) => {
            assert!(Instruction::$ident(Register::$r0(), Register::$r1()).is_ok());
        };
    }

    macro_rules! check_widths_err {
        ($ident:ident, $IDENT:ident, $r0:ident, $r1:ident) => {
            assert_eq!(
                Instruction::$ident(Register::$r0(), Register::$r1()), 
                Err(Error::OperandWidthMismatch(Instruction::$IDENT(Register::$r0(), Register::$r1())))
            );
        };
    }

    macro_rules! check_widths_case {
        ($name:ident, $ident:ident, $IDENT:ident) => {
            #[test]
            fn $ident() {
                check_widths_ok!($ident, rb0, rb1);
                check_widths_ok!($ident, r0, r1);
                check_widths_err!($ident, $IDENT, rb0, r1);
                check_widths_err!($ident, $IDENT, r0, rb1);
            }
        };
    }

    #[test]
    fn check_widths_c2r() {
        assert!(Instruction::movc2r(Value::byte(0), Register::rb0()).is_ok());
        assert!(Instruction::movc2r(Value::word(0), Register::r0()).is_ok());
        assert_eq!(Instruction::movc2r(Value::byte(0), Register::r0()), Err(Error::OperandWidthMismatch(Instruction::MovC2R(Value::byte(0), Register::r0()))));
        assert_eq!(Instruction::movc2r(Value::word(0), Register::rb0()), Err(Error::OperandWidthMismatch(Instruction::MovC2R(Value::word(0), Register::rb0()))));
    }

    check_widths_case!(check_widths_movr2r, movr2r, MovR2R);

    #[test]
    fn check_widths_movm2r() {
        check_widths_ok!(movm2r, r0, rb1);
        check_widths_err!(movm2r, MovM2R, r0, r1);
        check_widths_err!(movm2r, MovM2R, rb0, rb1);
        check_widths_err!(movm2r, MovM2R, rb0, r1);
    }

    #[test]
    fn check_widths_movr2m() {
        check_widths_ok!(movr2m, rb0, r1);
        check_widths_err!(movr2m, MovR2M, r0, r1);
        check_widths_err!(movr2m, MovR2M, rb0, rb1);
        check_widths_err!(movr2m, MovR2M, r0, rb1);
    }

    check_widths_case!(check_widths_add, add, Add);
    check_widths_case!(check_widths_sub, sub, Sub);

    #[test]
    fn all_different_opcodes() {
        macro_rules! b2b {
            ($ident:ident) => {
                Instruction::$ident(Register::rb0(), Register::rb1()).unwrap()
            };
        }

        macro_rules! w2w {
            ($ident:ident) => {
                Instruction::$ident(Register::r0(), Register::r1()).unwrap()
            };
        }

        // All instructions
        let all = vec![
            Instruction::nop(),

            Instruction::movc2r(Value::byte(1), Register::rb0()).unwrap(),
            Instruction::movc2r(Value::word(1), Register::r0()).unwrap(),
            b2b!(movr2r),
            w2w!(movr2r),
            Instruction::movm2r(Register::r0(), Register::rb1()).unwrap(),
            Instruction::movr2m(Register::rb0(), Register::r1()).unwrap(),

            b2b!(add),
            w2w!(add),
            b2b!(sub),
            w2w!(sub),
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
