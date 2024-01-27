pub enum Register {
    /// CPU information flags, 16-bits
    RINFO,

    /// Instruction pointer, 16-bits
    RIP, 

    /// Interrupt handler pointer, 16-bits
    RINT,

    /// Flags, always 16-bits
    Flags,

    /// General purpose register 16 or 8 bits depending on low
    R {
        /// Number, between 0 and 11
        number: u8,

        /// Is in low mode, only lower 8-bits
        low: bool
    },
}
