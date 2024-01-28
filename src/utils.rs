use crate::Instruction;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("invalid instruction, operand widths don't match: {0:?}")]
    OperandWidthMismatch(Instruction),
}
pub type Result<T> = std::result::Result<T, Error>;
