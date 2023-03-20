use crate::lexer::Span;
use crate::{from_token, token};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lit {
    Int(LitInt),
    Bool(LitBool),
    Str(LitStr),
    Char(LitChar),
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

// impl LitInt {
//     pub fn _parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
//         self.value.parse::<T>()
//     }
// }
