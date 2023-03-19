use std::fmt;
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum RegPreserved64 {
    Rbx,
    Rsp,
    Rbp,
    R12,
    R13,
    R14,
    R15,
}

impl fmt::Display for RegPreserved64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rbx => write!(f, "rbx"),
            Self::Rsp => write!(f, "rsp"),
            Self::Rbp => write!(f, "rbp"),
            Self::R12 => write!(f, "r12"),
            Self::R13 => write!(f, "r13"),
            Self::R14=> write!(f,  "r14"),
            Self::R15=> write!(f,  "r15"),
        }
    }
}
