use super::*;

impl Instruction {
    pub fn compile(&self) -> Vec<u8> {
        use Instruction::*;

        match self {
            Nop => vec![self.opcode(), 0x00],

            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nop() {
        assert_eq!(Instruction::Nop.compile(), vec![0x00, 0x00]);
    }
}
