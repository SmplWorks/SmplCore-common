use super::*;

impl Instruction {
    pub fn compile(&self) -> Vec<u8> {
        use Instruction::*;

        match self {
            Nop => vec![self.opcode(), 0x00],
            DB(_) => vec![self.opcode()],

            MovC2R(value, dest) => match value.width() {
                Width::Byte => vec![self.opcode(), dest.compile_dest(), value.value_byte(0), 0x00],
                Width::Word => vec![self.opcode(), dest.compile_dest(), value.value_byte(0), value.value_byte(1)],
            }

            MovR2R(src, dest) | MovM2R(src, dest) | MovR2M(src, dest) |
            Add(src, dest) | Sub(src, dest)
                => vec![self.opcode(), src.compile_with(dest)],

            Jmp(reg) => vec![self.opcode(), reg.compile_src()],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! case {
        ($ident:ident, $r0:ident, $r1:ident) => {
            let inst = Instruction::$ident(Register::$r0(), Register::$r1()).unwrap();
            let bytes = inst.compile();
            assert_eq!(bytes, vec![inst.opcode(), Register::$r0().compile_with(&Register::$r1())]);
            assert_eq!(bytes.len(), inst.len());
        };
    }

    macro_rules! case_two {
        ($ident:ident) => {
            #[test]
            fn $ident() {
                case!($ident, rb0, rb1);
                case!($ident, r0, r1);
            }
        };
    }

    #[test]
    fn nop() {
        let inst = Instruction::nop();
        let bytes = inst.compile();
        assert_eq!(bytes, vec![inst.opcode(), 0x00]);
        assert_eq!(bytes.len(), inst.len());
    }

    #[test]
    fn db() {
        let inst = Instruction::db(0xF3);
        let bytes = inst.compile();
        assert_eq!(bytes, vec![inst.opcode()]);
        assert_eq!(bytes.len(), inst.len());
    }

    #[test]
    fn movc2r() {
        let inst = Instruction::movc2r(Value::byte(0xF3), Register::rb0()).unwrap();
        let bytes = inst.compile();
        assert_eq!(bytes, vec![inst.opcode(), Register::rb0().compile_dest(), 0xF3, 0x00]);
        assert_eq!(bytes.len(), inst.len());

        let inst = Instruction::movc2r(Value::word(0xF337), Register::r0()).unwrap();
        let bytes = inst.compile();
        assert_eq!(bytes, vec![inst.opcode(), Register::r0().compile_dest(), 0x37, 0xF3]);
        assert_eq!(bytes.len(), inst.len());
    }

    case_two!(movr2r);

    #[test]
    fn movm2r() {
        case!(movm2r, r0, rb1);
    }

    #[test]
    fn movr2m() {
        case!(movr2m, rb0, r1);
    }
    
    case_two!(add);
    case_two!(sub);

    #[test]
    fn jmp() {
        let inst = Instruction::jmp(Register::r0()).unwrap();
        let bytes = inst.compile();
        assert_eq!(bytes, vec![inst.opcode(), Register::r0().compile_src()]);
        assert_eq!(bytes.len(), inst.len());
    }
}
