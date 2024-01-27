enum ValueType {
    Byte, Word, 
}
use ValueType::*;

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
