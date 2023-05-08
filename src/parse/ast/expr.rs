use super::{Ident, Lit, Op, keyword};
use crate::lexer::{Span, Token};
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Lit(ExprLit),
    Binary(ExprBinary),
    Call(ExprCall),
    Var(ExprVar),
    If(ExprIf),
    Block(ExprBlock),
    Return(ExprReturn),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(elit) => write!(f, "{elit}"),
            Self::Binary(ebin) => write!(f, "{ebin}"),
            Self::Call(ecall) => write!(f, "{ecall}"),
            Self::Var(evar) => write!(f, "{evar}"),
            Self::If(i) => write!(f, "{i}"),
            Self::Block(i) => write!(f, "{i}"),
            Self::Return(i) => write!(f, "{i}"),
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
            Self::If(i) => i.span(),
            Self::Block(i) => i.span(),
            Self::Return(i) => i.span(),
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
        Self::Var(ExprVar::new(name))
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

impl From<ExprIf> for Expr {
    fn from(expr: ExprIf) -> Self {
        Self::If(expr)
    }
}

impl From<ExprBlock> for Expr {
    fn from(expr: ExprBlock) -> Self {
        Self::Block(expr)
    }
}

impl From<ExprReturn> for Expr {
    fn from(expr: ExprReturn) -> Self {
        Self::Return(expr)
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
        Span::from((start, end))
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
    pub left_paran: super::CtrlLParan,
    pub args: Vec<Expr>,
    pub right_paran: super::CtrlRParan,
}

impl fmt::Display for ExprCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { caller, args, .. } = &self;
        let args = args.iter().map(|i| format!("{i}, ")).collect::<String>();
        write!(f, "({caller} ({args}))")
    }
}

impl ExprCall {
    pub fn new(
        caller: Box<Expr>,
        left_paran: super::CtrlLParan,
        args: Vec<Expr>,
        right_paran: super::CtrlRParan,
    ) -> Self {
        Self {
            caller,
            left_paran,
            args,
            right_paran,
        }
    }
    pub fn span(&self) -> Span {
        let start = self.caller.span();
        let end = self.right_paran.span();
        Span::from((start, end))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprVar {
    pub name: Ident,
}

impl fmt::Display for ExprVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name, .. } = &self;
        write!(f, "{name}")
    }
}

impl ExprVar {
    pub fn new(name: Ident) -> Self {
        Self { name }
    }

    pub fn span(&self) -> Span {
        self.name.span()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprIf {
    pub if_token: super::keyword::If,
    pub cond: Box<Expr>,
    pub then_branch: ExprBlock,
    pub else_branch: Option<(super::keyword::Else, Box<Expr>)>,
}

// TODO: implement display for ExprIf
impl fmt::Display for ExprIf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if unimplemented display")
    }
}

impl ExprIf {
    pub fn new(
        if_token: super::keyword::If,
        cond: Box<Expr>,
        then_branch: ExprBlock,
        else_branch: Option<(super::keyword::Else, Box<Expr>)>,
    ) -> Self {
        Self {
            if_token,
            cond,
            then_branch,
            else_branch,
        }
    }
    pub fn span(&self) -> Span {
        let start = self.if_token.span();
        let end = self
            .else_branch
            .as_ref()
            .map(|i| i.1.span())
            .unwrap_or(self.then_branch.span());
        Span::from((start, end))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprBlock {
    pub left_brace: super::CtrlLBrace,
    pub right_brace: super::CtrlRBrace,
    pub stmts: Vec<super::Statement>,
}

impl std::fmt::Display for ExprBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { stmts, .. } = &self;
        let stmts = stmts
            .iter()
            .map(|stmt| format!("{stmt}\n"))
            .collect::<String>();
        write!(f, "{stmts}")
    }
}

impl ExprBlock {
    pub fn new(
        left_brace: super::CtrlLBrace,
        right_brace: super::CtrlRBrace,
        stmts: Vec<super::Statement>,
    ) -> Self {
        Self {
            left_brace,
            right_brace,
            stmts,
        }
    }

    pub fn span(&self) -> Span {
        let start = self.left_brace.span();
        let end = self.right_brace.span();
        Span::from((start, end))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprReturn{
    pub ret: keyword::Return,
    pub expr: Box<Expr>,
}

impl ExprReturn {
    pub fn new(ret: keyword::Return, expr: Expr) -> Self {
        Self{
            ret,
            expr: Box::new(expr)
        }
    }

    pub fn span(&self) -> Span {
        let start = self.ret.span();
        let end = self.expr.span();
        Span::from((start, end))
    }
}

impl std::fmt::Display for ExprReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { ret, expr } = self;
        write!(f, "{ret} {expr}")
    }
}
