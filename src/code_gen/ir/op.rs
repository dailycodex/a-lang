use crate::lexer::TokenKind;
#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
    Equal,
    Grt,
    Les,
    Geq,
    Leq,
    Neq,
}

impl From<TokenKind> for Op {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            TokenKind::Star => Self::Mult,
            TokenKind::Slash => Self::Div,
            TokenKind::EqEq => Self::Equal,
            TokenKind::Grt => Self::Grt,
            TokenKind::Les => Self::Les,
            TokenKind::Geq => Self::Geq,
            TokenKind::Leq => Self::Leq,
            TokenKind::Neq => Self::Neq,
            _ => unimplemented!(),
        }
    }
}
