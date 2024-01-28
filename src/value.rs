#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueType {
    Byte, Word, 
}
use ValueType::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value(ValueType, u16);

impl Value {
    pub fn new_byte(value : u8) -> Self {
        Self(Byte, value as u16)
    }

    pub fn new_word(value : u16) -> Self {
        Self(Word, value)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.0, self.1)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::new_byte(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::new_word(value)
    }
}
