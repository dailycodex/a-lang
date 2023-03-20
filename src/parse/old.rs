use crate::lexer::{Span, Token};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Type {
    pub name: Name,
    pub span: Span,
}

impl From<&Token> for Type {
    fn from(value: &Token) -> Self {
        Self {
            name: value.into(),
            span: value.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Fn(ItemFn),
}

#[derive(Debug, Clone)]
pub struct Name {
    pub name: String,
    pub span: Span,
}

impl From<&Token> for Name {
    fn from(value: &Token) -> Self {
        Self {
            name: value.lexme(),
            span: value.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub kind: Type,
    pub span: Span,
}

impl From<(&Token, &Token)> for Param {
    fn from((name, kind): (&Token, &Token)) -> Self {
        let start_span = name.span();
        let end_span = kind.span();
        let span = Span::new(start_span.line, start_span.start, end_span.end);
        Self {
            name: name.lexme(),
            kind: kind.into(),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemFn {
    pub name: Name,
    pub params: Vec<Param>,
    pub block: Block,
    pub ret_type: Option<Type>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub stmt: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitInt {
    pub value: String,
    pub span: Span,
}

impl LitInt {
    pub fn _parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
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

impl Binary {
    pub fn span(&self) -> Span {
        let start = self.left.span();
        let end = self.right.span();
        Span::new(start.line, start.start, end.end)
    }
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

impl Lit {
    pub fn span(&self) -> Span {
        match self {
            Self::Int(int) => int.span,
            Self::Bool(bl) => bl.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprCall {
    pub caller: Box<Expr>,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl ExprCall {
    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Name,
    pub span: Span,
}

impl Var {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl From<&Token> for Var {
    fn from(value: &Token) -> Self {
        Self {
            name: value.into(),
            span: value.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Lit(Lit),
    Binary(Binary),
    Call(ExprCall),
    Var(Var),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Lit(i) => i.span(),
            Self::Binary(i) => i.span(),
            Self::Call(i) => i.span(),
            Self::Var(i) => i.span(),
        }
    }
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

impl From<Var> for Expr {
    fn from(var: Var) -> Self {
        Self::Var(var)
    }
}
