use super::*;

impl Instruction {
    pub fn compile(&self) -> Vec<u8> {
        use Instruction::*;

        match self {
            Nop => vec![self.opcode(), 0x00],
            DB(_) => vec![self.opcode()],

            MovC2R(value, dest) |
            AddC2R(value, dest) | SubC2R(value, dest)
                => match value.width() {
                Width::Byte => vec![self.opcode(), dest.compile_dest(), value.value_byte(0), 0x00],
                Width::Word => vec![self.opcode(), dest.compile_dest(), value.value_byte(0), value.value_byte(1)],
            }

            MovR2R(src, dest) | MovM2R(src, dest) | MovR2M(src, dest) |
            AddR2R(src, dest) | SubR2R(src, dest)
                => vec![self.opcode(), src.compile_with(dest)],

            AJmp(reg) | Jmp(reg)
                => vec![self.opcode(), reg.compile_src()],

            _ => todo!("{self:?}")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! case_r {
        ($ident:ident, $reg:ident, $as:ident) => {
            let inst = Instruction::$ident(Register::$reg()).unwrap();
            let bytes = inst.compile();
            assert_eq!(bytes, vec![inst.opcode(), Register::$reg().$as()]);
            assert_eq!(bytes.len(), inst.len().into());
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

    #[test]
    fn nop() {
        let inst = Instruction::nop();
        let bytes = inst.compile();
        assert_eq!(bytes, vec![inst.opcode(), 0x00]);
        assert_eq!(bytes.len(), inst.len().into());
    }

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
    
    case_two_c2r!(addc2r);
    case_two_r2r!(addr2r);
    case_two_c2r!(subc2r);
    case_two_r2r!(subr2r);

    #[test]
    fn ajmp() {
        case_r!(ajmp, r0, compile_src);
    }

    #[test]
    fn jmp() {
        case_r!(jmp, r0, compile_src);
    }
}
