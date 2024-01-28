#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueType {
    Byte, Word, 
}
use ValueType::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value {
    t : ValueType,
    value : u16,
}

impl Value {
    pub fn new_byte(value : u8) -> Self {
        Self { t : Byte, value: value as u16 }
    }

    pub fn new_word(value : u16) -> Self {
        Self { t : Word, value }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {{ value: {:?} }}", self.t, self.value)
    }
}
