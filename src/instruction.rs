use crate::{Register, Value};

pub enum Instruction {
    // Misc
    /// No operation
    Nop,

    // Memory manipulation
    /// Move constant to register
    MovC2R { value: Value, dest: Register },

    /// Move register to register
    MovR2R { src: Register, dest: Register },

    /// Move from memory to register
    MovM2R { src: Register, dest: Register },

    /// Move from register to memory
    MovR2M { src: Register, dest: Register },
    
    // Arithmetic
    /// Add two registers
    Add { src: Register, dest: Register },

    /// Subtract two registers
    Sub { src: Register, dest: Register },
}
