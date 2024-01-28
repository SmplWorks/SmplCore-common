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
