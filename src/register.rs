#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    Byte, Word, 
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register {
    /// CPU information flags, 16-bits
    RINFO,

    /// Instruction pointer, 16-bits
    RIP, 

    /// Interrupt handler pointer, 16-bits
    RINT,

    /// Flags, always 16-bits
    Flags,

    /// General purpose register
    R(Width, u8),
}

macro_rules! r_const {
    ($ident:ident, $width:ident, $number:literal) => {
        pub fn $ident() -> Self {
            Self::R(Width::$width, $number)
        }
    };
}

impl Register {
    r_const!(rb0, Byte, 0);
    r_const!(r0, Word, 0);
    r_const!(rb1, Byte, 1);
    r_const!(r1, Word, 1);
    r_const!(rb2, Byte, 2);
    r_const!(r2, Word, 2);
    r_const!(rb3, Byte, 3);
    r_const!(r3, Word, 3);
    r_const!(rb4, Byte, 4);
    r_const!(r4, Word, 4);
    r_const!(rb5, Byte, 5);
    r_const!(r5, Word, 5);
    r_const!(rb6, Byte, 6);
    r_const!(r6, Word, 6);
    r_const!(rb7, Byte, 7);
    r_const!(r7, Word, 7);
    r_const!(rb8, Byte, 8);
    r_const!(r8, Word, 8);
    r_const!(rb9, Byte, 9);
    r_const!(r9, Word, 9);
    r_const!(rb10, Byte, 10);
    r_const!(r10, Word, 10);
    r_const!(rb11, Byte, 11);
    r_const!(r11, Word, 11);

    pub fn width(&self) -> Width {
        use Register::*;
        match self {
            RINFO | RIP | RINT | Flags => Width::Word,
            R(w, _) => *w,
        }
    }

    pub fn is_writable(&self) -> bool {
        use Register::*;
        match self {
            RINFO | RIP | RINT | Flags => false,
            R(_, _) => true,
        }
    }

    pub fn compile_src(&self) -> u8 {
        use Register::*;
        match self {
            RINFO => 0,
            RIP => 1,
            RINT => 2,
            Flags => 3,
            R(_, number) => 4 + number,
        }
    }

    pub fn compile_dest(&self) -> u8 {
        self.compile_src() << 4
    }

    pub fn compile_with(&self, reg : &Register) -> u8 {
        self.compile_src() | reg.compile_dest()
    }

    pub fn from_dest(width : Width, dest : u8) -> Self {
        Self::from_src(width, dest >> 4)
    }

    pub fn from_src(width : Width, src : u8) -> Self {
        use Register::*;
        match src & 0xF {
            0 => RINFO,
            1 => RIP,
            2 => RINT,
            3 => Flags,
            n => R(width, n - 4),
        }
    }
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register::*;
        match self {
            RINFO => write!(f, "RINFO"),
            RIP => write!(f, "RIP"),
            RINT => write!(f, "RINT"),
            Flags => write!(f, "Flags"),
            R(width, number) => {
                let middle = match width {
                    Width::Byte => "b",
                    Width::Word => "",
                };
                write!(f, "R{}{}", middle, number)
            },
        }
    }
}

#[cfg(test)]
#[test]
fn compile_from_src() {
    for r in vec![Register::RINFO, Register::RIP, Register::RIP, Register::Flags, Register::r0(), Register::r1(), Register::r2(), Register::r3(), Register::r4(), Register::r5(), Register::r6(), Register::r7(), Register::r8(), Register::r9(), Register::r10(), Register::r11()] {
        let src = r.compile_src();
        assert_eq!(Register::from_src(r.width(), src), r);
    }
}
