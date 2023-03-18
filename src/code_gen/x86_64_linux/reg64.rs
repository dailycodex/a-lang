use std::fmt;
use super::Reg8;

#[repr(usize)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Reg64 {
    Rax = 0,
    Rdi,
    Rsi,
    Rdx,
    Rcx,
    R8,
    R9,
    R10,
    R11,
}

impl Reg64 {
    pub fn lower_8bit(&self) -> Reg8 {
        match self {
            Self::Rax => Reg8::Al,
            Self::Rdi => Reg8::Dil,
            Self::Rsi => Reg8::Sil,
            Self::Rdx => Reg8::Dl,
            Self::Rcx => Reg8::Cl,
            Self::R8 => Reg8::R8b,
            Self::R9 => Reg8::R9b,
            Self::R10 => Reg8::R10b,
            Self::R11 => Reg8::R11b,
        }
    }
}

impl fmt::Display for Reg64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rax => write!(f, "rax"),
            Self::Rdi => write!(f, "rdi"),
            Self::Rsi => write!(f, "rsi"),
            Self::Rdx => write!(f, "rdx"),
            Self::Rcx => write!(f, "rcx"),
            Self::R8 => write!(f, "r8"),
            Self::R9 => write!(f, "r9"),
            Self::R10 => write!(f, "r10"),
            Self::R11 => write!(f, "r11"),
        }
    }
}
