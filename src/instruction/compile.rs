use super::*;

impl Instruction {
    pub fn compile(&self) -> Vec<u8> {
        use Instruction::*;

        match self {
            Nop | Ret | Cli => vec![self.opcode(), 0x00],
            DB(_) => vec![self.opcode()],

            MovC2R(value, dest) |
            AddC2R(value, dest) | SubC2R(value, dest) | AndC2R(value, dest) | OrC2R(value, dest) | CmpC2R(value, dest)
                => match value.width() {
                Width::Byte => vec![self.opcode(), dest.compile_dest(), value.value_byte(0), 0x00],
                Width::Word => vec![self.opcode(), dest.compile_dest(), value.value_byte(0), value.value_byte(1)],
            }

            MovR2R(src, dest) | MovM2R(src, dest) | MovR2M(src, dest) |
            AddR2R(src, dest) | SubR2R(src, dest) | AndR2R(src, dest) | OrR2R(src, dest) | CmpR2R(src, dest)
                => vec![self.opcode(), src.compile_with(dest)],

            Push(reg) | Pop(reg) |
            Not(reg) |
            AJmp(reg) | Jmp(reg) | Jeq(reg) | Jneq(reg) | Jlt(reg) | Jgt(reg) | Jleq(reg) | Jgeq(reg) | Jo(reg) | Jno(reg) | CallR(reg) |
            Int(reg) | Sti(reg)
                => vec![self.opcode(), reg.compile_src()],

            Shl(value, dest) | Shr(value, dest) | Shre(value, dest)
                => vec![self.opcode(), value.value_byte(0) | dest.compile_dest()],

            CallC(value)
                => vec![self.opcode(), 0x00, value.value_byte(0), value.value_byte(1)],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;


    macro_rules! case_no {
        ($ident:ident) => {
            #[test]
            fn $ident() {
                let inst = Instruction::$ident();
                let bytes = inst.compile();
                assert_eq!(bytes, vec![inst.opcode(), 0x00]);
                assert_eq!(bytes.len(), inst.len().into());
            }
        };
    }

    macro_rules! case_r {
        ($ident:ident, $reg:ident) => {
            let inst = Instruction::$ident(Register::$reg()).unwrap();
            let bytes = inst.compile();
            assert_eq!(bytes, vec![inst.opcode(), Register::$reg().compile_src()]);
            assert_eq!(bytes.len(), inst.len().into());
        };
    }

    macro_rules! case_one_r {
        ($ident:ident) => {
            #[test]
            fn $ident() {
                case_r!($ident, r0);
            }
        };
    }

    macro_rules! case_two_r {
        ($ident:ident) => {
            #[test]
            fn $ident() {
                case_r!($ident, rb0);
                case_r!($ident, r0);
            }
        };
    }

    macro_rules! case_c2r {
        ($ident:ident, $width:ident, $value:literal, $dest:ident) => {
            let inst = Instruction::$ident(Value::$width($value), Register::$dest()).unwrap();
            let bytes = inst.compile();
            assert_eq!(bytes, vec![inst.opcode(), Register::$dest().compile_dest(), Value::$width($value).value_byte(0), Value::$width($value).value_byte(1)]);
            assert_eq!(bytes.len(), inst.len().into());
        };
    }

    macro_rules! case_r2r {
        ($ident:ident, $r0:ident, $r1:ident) => {
            let inst = Instruction::$ident(Register::$r0(), Register::$r1()).unwrap();
            let bytes = inst.compile();
            assert_eq!(bytes, vec![inst.opcode(), Register::$r0().compile_with(&Register::$r1())]);
            assert_eq!(bytes.len(), inst.len().into());
        };
    }

    macro_rules! case_two_c2r {
        ($ident:ident) => {
            #[test]
            fn $ident() {
                case_c2r!($ident, byte, 0xF3, rb1);
                case_c2r!($ident, word, 0xF337, r1);
            }
        };
    }

    macro_rules! case_two_r2r {
        ($ident:ident) => {
            #[test]
            fn $ident() {
                case_r2r!($ident, rb0, rb1);
                case_r2r!($ident, r0, r1);
            }
        };
    }

    macro_rules! case_shift {
        ($ident:ident) => {
            #[test]
            fn $ident() {
                let inst = Instruction::$ident(Value::byte(8), Register::rb0()).unwrap();
                let bytes = inst.compile();
                assert_eq!(bytes, vec![inst.opcode(), 8 | Register::rb0().compile_dest()]);
                assert_eq!(bytes.len(), inst.len().into());

                let inst = Instruction::$ident(Value::byte(16), Register::r0()).unwrap();
                let bytes = inst.compile();
                assert_eq!(bytes, vec![inst.opcode(), 16 | Register::rb0().compile_dest()]);
                assert_eq!(bytes.len(), inst.len().into());
            }
        };
    }

    case_no!(nop);

    #[test]
    fn db() {
        let inst = Instruction::db(0xF3);
        let bytes = inst.compile();
        assert_eq!(bytes, vec![inst.opcode()]);
        assert_eq!(bytes.len(), inst.len().into());
    }

    case_two_c2r!(movc2r);
    case_two_r2r!(movr2r);

    #[test]
    fn movm2r() {
        case_r2r!(movm2r, r0, rb1);
    }

    #[test]
    fn movr2m() {
        case_r2r!(movr2m, rb0, r1);
    }

    case_one_r!(push);
    case_one_r!(pop);

    case_two_c2r!(addc2r);
    case_two_r2r!(addr2r);
    case_two_c2r!(subc2r);
    case_two_r2r!(subr2r);
    case_two_r!(not);
    case_two_c2r!(andc2r);
    case_two_r2r!(andr2r);
    case_two_c2r!(orc2r);
    case_two_r2r!(orr2r);
    case_shift!(shl);
    case_shift!(shr);
    case_shift!(shre);
    case_two_c2r!(cmpc2r);
    case_two_r2r!(cmpr2r);

    case_one_r!(ajmp);
    case_one_r!(jmp);
    case_one_r!(jeq);
    case_one_r!(jneq);
    case_one_r!(jlt);
    case_one_r!(jgt);
    case_one_r!(jleq);
    case_one_r!(jgeq);
    case_one_r!(jo);
    case_one_r!(jno);

    #[test]
    fn callc() {
        let inst = Instruction::callc(Value::word(0xF337)).unwrap();
        let bytes = inst.compile();
        assert_eq!(bytes, vec![inst.opcode(), 0x00, 0x37, 0xF3]);
        assert_eq!(bytes.len(), inst.len().into());
    }

    case_one_r!(callr);
    case_no!(ret);

    case_one_r!(int);
    case_one_r!(sti);
    case_no!(cli);
}
