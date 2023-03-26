use super::{Ident, Lit, Op};
use crate::lexer::{Span, Token};
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Lit(ExprLit),
    // Block(ExprBlock),
    Binary(ExprBinary),
    Call(ExprCall),
    Var(ExprVar),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(elit) => write!(f, "{elit}"),
            Self::Binary(ebin) => write!(f, "{ebin}"),
            Self::Call(ecall) => write!(f, "{ecall}"),
            Self::Var(evar) => write!(f, "{evar}"),
        }
    }
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

impl From<super::LitInt> for Expr {
    fn from(value: super::LitInt) -> Self {
        Self::Lit(ExprLit {
            lit: Lit::from(value),
        })
    }
}

impl From<super::LitBool> for Expr {
    fn from(value: super::LitBool) -> Self {
        Self::Lit(ExprLit {
            lit: Lit::from(value),
        })
    }
}

impl From<super::LitStr> for Expr {
    fn from(value: super::LitStr) -> Self {
        Self::Lit(ExprLit {
            lit: Lit::from(value),
        })
    }
}

impl From<super::LitChar> for Expr {
    fn from(value: super::LitChar) -> Self {
        Self::Lit(ExprLit {
            lit: Lit::from(value),
        })
    }
}

impl From<Ident> for Expr {
    fn from(name: Ident) -> Self {
        let span = name.span;
        Self::Var(ExprVar { name, span })
    }
}

impl From<ExprLit> for Expr {
    fn from(expr: ExprLit) -> Self {
        Self::Lit(expr)
    }
}

impl From<ExprBinary> for Expr {
    fn from(expr: ExprBinary) -> Self {
        Self::Binary(expr)
    }
}

impl From<ExprCall> for Expr {
    fn from(expr: ExprCall) -> Self {
        Self::Call(expr)
    }
}

impl From<ExprVar> for Expr {
    fn from(expr: ExprVar) -> Self {
        Self::Var(expr)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprLit {
    pub lit: Lit,
}

impl fmt::Display for ExprLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { lit } = &self;
        write!(f, "{lit}")
    }
}

impl From<Lit> for ExprLit {
    fn from(lit: Lit) -> Self {
        Self { lit }
    }
}

impl ExprLit {
    pub fn span(&self) -> Span {
        self.lit.span()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: Op,
}

impl fmt::Display for ExprBinary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { left, right, op } = &self;
        write!(f, "({op} {left} {right})")
    }
}

impl ExprBinary {
    pub fn span(&self) -> Span {
        let start = self.left.span();
        let end = self.right.span();
        Span::new(start.line, start.start, end.end)
    }
}

impl From<(Expr, Expr, Op)> for ExprBinary {
    fn from((lhs, rhs, op): (Expr, Expr, Op)) -> Self {
        Self {
            left: Box::new(lhs),
            right: Box::new(rhs),
            op,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprCall {
    pub caller: Box<Expr>,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl fmt::Display for ExprCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { caller, args, .. } = &self;
        let args = args.iter().map(ToString::to_string).collect::<String>();
        write!(f, "({caller} {args})")
    }
}

impl ExprCall {
    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprVar {
    pub name: Ident,
    pub span: Span,
}

impl fmt::Display for ExprVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, .. } = &self;
        write!(f, "{name}")
    }
}

impl ExprVar {
    pub fn span(&self) -> Span {
        self.span
    }
}
