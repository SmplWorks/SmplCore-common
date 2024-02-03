use crate::Instruction;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("invalid instruction, operand widths don't match: {0:?}")]
    OperandWidthMismatch(Instruction),

    #[error("invalid instruction, destiny operand isn't writable: {0:?}")]
    DestOperandNotWritable(Instruction),

    #[error("invalid register: {0:?}")]
    InvalidRegister(String),
}
pub type Result<T> = std::result::Result<T, Error>;
