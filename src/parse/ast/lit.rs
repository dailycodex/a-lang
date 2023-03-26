use crate::lexer::Span;
use crate::{from_token, token};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lit {
    Int(LitInt),
    Bool(LitBool),
    Str(LitStr),
    Char(LitChar),
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{i}"),
            Self::Bool(i) => write!(f, "{i}"),
            Self::Str(i) => write!(f, "{i}"),
            Self::Char(i) => write!(f, "{i}"),
        }
    }
}

impl Lit {
    pub fn span(&self) -> Span {
        match self {
            Self::Int(i) => i.span,
            Self::Bool(i) => i.span,
            Self::Str(i) => i.span,
            Self::Char(i) => i.span,
        }
    }
}

token!(LitInt);
token!(LitBool);
token!(LitStr);
token!(LitChar);

from_token!(Lit, Int, LitInt);
from_token!(Lit, Bool, LitBool);
from_token!(Lit, Str, LitStr);
from_token!(Lit, Char, LitChar);

impl LitInt {
    pub fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        self.value.parse::<T>()
    }
}

impl LitBool {
    pub fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        self.value.parse::<T>()
    }
}
