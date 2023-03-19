use super::{Reg64, Reg8, RegPreserved64};
use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86Reg {
    Reg64(Reg64),
    Reg8(Reg8),
    RegPre64(RegPreserved64),
}

impl X86Reg {
    pub fn lower_8bit(&self) -> Self {
        match self {
            Self::Reg64(reg) => Self::Reg8(reg.lower_8bit()),
            _ => *self,
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            X86Reg::Reg64(r) => *r as usize,
            X86Reg::Reg8(r) => *r as usize,
            X86Reg::RegPre64(r) => unimplemented!(),
        }
    }
}

impl From<Reg64> for X86Reg {
    fn from(value: Reg64) -> Self {
        Self::Reg64(value)
    }
}

impl From<Reg8> for X86Reg {
    fn from(value: Reg8) -> Self {
        Self::Reg8(value)
    }
}

impl From<RegPreserved64> for X86Reg {
    fn from(value: RegPreserved64) -> Self {
        Self::RegPre64(value)
    }
}

impl fmt::Display for X86Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reg64(reg) => write!(f, "{reg}"),
            Self::Reg8(reg) => write!(f, "{reg}"),
            Self::RegPre64(reg) => write!(f, "{reg}"),
        }
    }
}
