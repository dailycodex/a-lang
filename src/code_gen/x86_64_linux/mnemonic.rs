use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    Move,
    MoveZx,
    Add,
    Sub,
    Mul,
    Div,
    Cmp,
    SetEqual,
    SetGreater,
    SetLesser,
    SetGreaterEqual,
    SetLesserEqual,
    SetNotEqual,
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Move => write!(f, "mov"),
            Self::MoveZx => write!(f, "movzx"),
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Mul => write!(f, "imul"),
            Self::Div => write!(f, "idiv"),
            Self::Cmp => write!(f, "cmp"),
            Self::SetEqual => write!(f, "sete"),
            Self::SetGreater => write!(f, "setg"),
            Self::SetLesser => write!(f, "setl"),
            Self::SetGreaterEqual => write!(f, "setge"),
            Self::SetLesserEqual => write!(f, "setle"),
            Self::SetNotEqual => write!(f, "setne"),
        }
    }
}
