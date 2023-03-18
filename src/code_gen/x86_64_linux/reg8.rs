use std::fmt;

#[repr(usize)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Reg8 {
    Al = 0,
    Dil,
    Sil,
    Dl,
    Cl,
    R8b,
    R9b,
    R10b,
    R11b,
}

impl fmt::Display for Reg8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Al => write!(f, "al"),
            Self::Dil => write!(f, "ril"),
            Self::Sil => write!(f, "sil"),
            Self::Dl => write!(f, "dl"),
            Self::Cl => write!(f, "cl"),
            Self::R8b => write!(f, "r8b"),
            Self::R9b => write!(f, "r9b"),
            Self::R10b => write!(f, "r10b"),
            Self::R11b => write!(f, "r11b"),
        }
    }
}
