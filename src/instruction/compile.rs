use super::*;

impl Instruction {
    pub fn compile(&self) -> Vec<u8> {
        use Instruction::*;

        match self {
            Nop => vec![self.opcode(), 0x00],

            MovC2R(value, dest) => match value.width() {
                Width::Byte => vec![self.opcode(), value.value_byte(0) | dest.compile_dest()],
                Width::Word => vec![self.opcode(), dest.compile_dest(), value.value_byte(0), value.value_byte(1)],
            }

            MovR2R(src, dest) | MovM2R(src, dest) | MovR2M(src, dest) |
            Add(src, dest) | Sub(src, dest)
                => vec![self.opcode(), src.compile_with(dest)],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! case {
        ($ident:ident, $r0:ident, $r1:ident) => {
            let inst = Instruction::$ident(Register::$r0(), Register::$r1()).unwrap();
            assert_eq!(inst.compile(), vec![inst.opcode(), Register::$r0().compile_with(&Register::$r1())]);
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
        assert_eq!(inst.compile(), vec![inst.opcode(), 0x00]);
    }

    #[test]
    fn movc2r() {
        let inst = Instruction::movc2r(Value::byte(0xF3), Register::rb0()).unwrap();
        assert_eq!(inst.compile(), vec![inst.opcode(), 0xF3 | Register::rb0().compile_dest()]);

        let inst = Instruction::movc2r(Value::word(0xF337), Register::r0()).unwrap();
        assert_eq!(inst.compile(), vec![inst.opcode(), Register::r0().compile_dest(), 0x37, 0xF3]);
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
}
