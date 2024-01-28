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

            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nop() {
        assert_eq!(Instruction::nop().compile(), vec![0x00, 0x00]);
    }

    #[test]
    fn movc2r() {
        let inst = Instruction::movc2r(Value::byte(0xF3), Register::rb0()).unwrap();
        assert_eq!(inst.compile(), vec![inst.opcode(), 0xF3 | Register::rb0().compile_dest()]);

        let inst = Instruction::movc2r(Value::word(0xF337), Register::r0()).unwrap();
        assert_eq!(inst.compile(), vec![inst.opcode(), Register::r0().compile_dest(), 0x37, 0xF3]);
    }
}
