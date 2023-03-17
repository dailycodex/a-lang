use crate::span::Span;
use crate::token::Token;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LitInt {
    pub value: String,
    pub span: Span,
}

impl LitInt {
    pub fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        self.value.parse::<T>()
    }
}

impl From<&Token> for LitInt {
    fn from(token: &Token) -> Self {
        Self {
            value: token.lexme().to_string(),
            span: token.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LitBool {
    pub value: String,
    pub span: Span,
}

impl From<&Token> for LitBool {
    fn from(token: &Token) -> Self {
        Self {
            value: token.lexme().to_string(),
            span: token.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: Token,
}

impl From<(Expr, Expr, &Token)> for Binary {
    fn from((left, right, op): (Expr, Expr, &Token)) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
            op: op.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Lit {
    Int(LitInt),
    Bool(LitBool),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Lit(Lit),
    Binary(Binary),
}

impl From<LitBool> for Expr {
    fn from(lit_bool: LitBool) -> Self {
        Self::Lit(Lit::Bool(lit_bool))
    }
}

impl From<LitInt> for Expr {
    fn from(lit_int: LitInt) -> Self {
        Self::Lit(Lit::Int(lit_int))
    }
}

impl From<Binary> for Expr {
    fn from(bin: Binary) -> Self {
        Self::Binary(bin)
    }
}
