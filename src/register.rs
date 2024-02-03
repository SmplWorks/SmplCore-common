use crate::utils::{Error, Result};

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

    pub fn from_str(s : &str) -> Result<Self> {
        match &*s.to_lowercase() {
            "rinfo" => Ok(Self::RINFO),
            "rip" => Ok(Self::RIP),
            "rint" => Ok(Self::RINT),
            "flags" => Ok(Self::Flags),
            "r0" => Ok(Self::r0()),
            "rb0" => Ok(Self::rb0()),
            "r1" => Ok(Self::r1()),
            "rb1" => Ok(Self::rb1()),
            "r2" => Ok(Self::r2()),
            "rb2" => Ok(Self::rb2()),
            "r3" => Ok(Self::r3()),
            "rb3" => Ok(Self::rb3()),
            "r4" => Ok(Self::r4()),
            "rb4" => Ok(Self::rb4()),
            "r5" => Ok(Self::r5()),
            "rb5" => Ok(Self::rb5()),
            "r6" => Ok(Self::r6()),
            "rb6" => Ok(Self::rb6()),
            "r7" => Ok(Self::r7()),
            "rb7" => Ok(Self::rb7()),
            "r8" => Ok(Self::r8()),
            "rb8" => Ok(Self::rb8()),
            "r9" => Ok(Self::r9()),
            "rb9" => Ok(Self::rb9()),
            "r10" => Ok(Self::r10()),
            "rb10" => Ok(Self::rb10()),
            "r11" => Ok(Self::r11()),
            "rb11" => Ok(Self::rb11()),
            _ => Err(Error::InvalidRegister(s.to_string())),
        }
    }

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

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compile_from_src() {
        for r in vec![Register::RINFO, Register::RIP, Register::RIP, Register::Flags, Register::r0(), Register::r1(), Register::r2(), Register::r3(), Register::r4(), Register::r5(), Register::r6(), Register::r7(), Register::r8(), Register::r9(), Register::r10(), Register::r11()] {
            let src = r.compile_src();
            assert_eq!(Register::from_src(r.width(), src), r);
        }
    }

    #[test]
    fn from_str() {
        for r in vec![Register::RINFO, Register::RIP, Register::RIP, Register::Flags, Register::r0(), Register::r1(), Register::r2(), Register::r3(), Register::r4(), Register::r5(), Register::r6(), Register::r7(), Register::r8(), Register::r9(), Register::r10(), Register::r11()] {
            assert_eq!(Register::from_str(&r.to_string()), Ok(r));
        }
    }
}
