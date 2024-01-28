#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid register number (expected number between 0 and 11, found: {0})")]
    InvalidRegisterNumber(u8),
}
pub type Result<T> = std::result::Result<T, Error>;
