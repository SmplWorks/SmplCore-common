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
        fn $name() {
            check_widths_ok!($ident, rb0, rb1);
            check_widths_ok!($ident, r0, r1);
            check_widths_err!($ident, $IDENT, rb0, r1);
            check_widths_err!($ident, $IDENT, r0, rb1);
        }
    };
}

macro_rules! check_widths_case_c2r {
    ($name:ident, $ident:ident, $IDENT:ident) => {
        #[test]
        fn $name() {
            assert!(Instruction::$ident(Value::byte(0), Register::rb0()).is_ok());
            assert!(Instruction::$ident(Value::word(0), Register::r0()).is_ok());
            assert_eq!(Instruction::$ident(Value::byte(0), Register::r0()), Err(Error::OperandWidthMismatch(Instruction::$IDENT(Value::byte(0), Register::r0()))));
            assert_eq!(Instruction::$ident(Value::word(0), Register::rb0()), Err(Error::OperandWidthMismatch(Instruction::$IDENT(Value::word(0), Register::rb0()))));
        }
    };
}

check_widths_case_c2r!(check_widths_movc2r, movc2r, MovC2R);
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

check_widths_case_c2r!(check_widths_addc2r, addc2r, AddC2R);
check_widths_case!(check_widths_addr2r, addr2r, AddR2R);
check_widths_case_c2r!(check_widths_subc2r, subc2r, SubC2R);
check_widths_case!(check_widths_subr2r, subr2r, SubR2R);

#[test]
fn all_different_opcodes() {
    macro_rules! b {
        ($ident:ident) => {
            Instruction::$ident(Register::rb0()).unwrap()
        };
    }

    macro_rules! w {
        ($ident:ident) => {
            Instruction::$ident(Register::r0()).unwrap()
        };
    }

    macro_rules! cb2b {
        ($ident:ident) => {
            Instruction::$ident(Value::byte(0), Register::rb1()).unwrap()
        };
    }

    macro_rules! b2b {
        ($ident:ident) => {
            Instruction::$ident(Register::rb0(), Register::rb1()).unwrap()
        };
    }

    macro_rules! cw2w {
        ($ident:ident) => {
            Instruction::$ident(Value::word(0), Register::r1()).unwrap()
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

        cb2b!(movc2r),
        cw2w!(movc2r),
        b2b!(movr2r),
        w2w!(movr2r),
        Instruction::movm2r(Register::r0(), Register::rb1()).unwrap(),
        Instruction::movr2m(Register::rb0(), Register::r1()).unwrap(),
        w!(push),
        w!(pop),

        cb2b!(addc2r),
        cw2w!(addc2r),
        b2b!(addr2r),
        w2w!(addr2r),
        cb2b!(subc2r),
        cw2w!(subc2r),
        b2b!(subr2r),
        w2w!(subr2r),
        b!(not),
        w!(not),
        cb2b!(andc2r),
        cw2w!(andc2r),
        b2b!(andr2r),
        w2w!(andr2r),
        cb2b!(orc2r),
        cw2w!(orc2r),
        b2b!(orr2r),
        w2w!(orr2r),
        cb2b!(shl),
        cw2w!(shl),
        cb2b!(shr),
        cw2w!(shr),
        cb2b!(shre),
        cw2w!(shre),
        cb2b!(cmpc2r),
        cw2w!(cmpc2r),
        b2b!(cmpr2r),
        w2w!(cmpr2r),

        w!(ajmp),
        w!(jmp),
        w!(jeq),
        w!(jneq),
        w!(jlt),
        w!(jgt),
        w!(jleq),
        w!(jgeq),
        w!(jo),
        w!(jno),
        Instruction::callc(Value::word(0)).unwrap(),
        w!(callr),
        Instruction::Ret,
        w!(int), 
        w!(sti), 
        Instruction::Cli,
    ];

    for inst0 in all.iter() {
        for inst1 in all.iter() {
            if inst0 != inst1 {
                assert_ne!(inst0.opcode(), inst1.opcode(), "{:?} {:?}", inst0, inst1);
            }
        }
    }
}
