mod expr;
mod item;
pub mod keyword;
mod lit;

use crate::lexer::Span;
pub use expr::{Expr, ExprBinary, ExprBlock, ExprCall, ExprIf, ExprLit, ExprVar};
pub use item::{Item, ItemFn};
pub use lit::{Lit, LitBool, LitChar, LitInt, LitStr};

#[macro_export]
macro_rules! token {
    ($name:ident) => {
        #[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name {
            pub value: String,
            pub span: crate::lexer::Span,
        }

        impl $name {
            pub fn new(value: impl Into<String>, span: crate::lexer::Span) -> Self {
                Self {
                    value: value.into(),
                    span,
                }
            }
        }

        impl crate::lexer::Token for $name {
            fn new(value: String, span: crate::lexer::Span) -> Self {
                Self { value, span }
            }
            fn value(&self) -> String {
                self.value.to_string()
            }
            fn span(&self) -> crate::lexer::Span {
                self.span
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} '{}' {:?}", stringify!($name), self.value, self.span)
            }
        }
        impl $name {
            pub fn parse<T: std::str::FromStr>(&self) -> Result<T, <T as std::str::FromStr>::Err> {
                self.value.parse::<T>()
            }
        }
    };
}

#[macro_export]
macro_rules! from_token {
    ($name:ident, $var:ident, $from:ident) => {
        impl From<$from> for $name {
            fn from(value: $from) -> Self {
                Self::$var(value)
            }
        }

        impl From<&$from> for $name {
            fn from(value: &$from) -> Self {
                Self::$var(value.clone())
            }
        }
    };
}

token!(Ident);
token!(OpAdd);
token!(OpSub);
token!(OpMul);
token!(OpDiv);
token!(OpGrt);
token!(OpLes);
token!(OpGeq);
token!(OpLeq);
token!(OpNeq);
token!(OpNot);
token!(OpEqual);
token!(OpEqualEqual);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Add(OpAdd),
    Sub(OpSub),
    Mul(OpMul),
    Div(OpDiv),
    Grt(OpGrt),
    Les(OpLes),
    Geq(OpGeq),
    Leq(OpLeq),
    Neq(OpNeq),
    Not(OpNot),
    Equal(OpEqual),
    EqualEqual(OpEqualEqual),
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(op) => write!(f, "{op}"),
            Self::Sub(op) => write!(f, "{op}"),
            Self::Mul(op) => write!(f, "{op}"),
            Self::Div(op) => write!(f, "{op}"),
            Self::Grt(op) => write!(f, "{op}"),
            Self::Les(op) => write!(f, "{op}"),
            Self::Geq(op) => write!(f, "{op}"),
            Self::Leq(op) => write!(f, "{op}"),
            Self::Neq(op) => write!(f, "{op}"),
            Self::Not(op) => write!(f, "{op}"),
            Self::Equal(op) => write!(f, "{op}"),
            Self::EqualEqual(op) => write!(f, "{op}"),
        }
    }
}

from_token!(Op, Add, OpAdd);
from_token!(Op, Sub, OpSub);
from_token!(Op, Mul, OpMul);
from_token!(Op, Div, OpDiv);
from_token!(Op, Grt, OpGrt);
from_token!(Op, Les, OpLes);
from_token!(Op, Geq, OpGeq);
from_token!(Op, Leq, OpLeq);
from_token!(Op, Neq, OpNeq);
from_token!(Op, Not, OpNot);
from_token!(Op, Equal, OpEqual);
from_token!(Op, EqualEqual, OpEqualEqual);

token!(CtrlStar);
token!(CtrlSlash);
token!(CtrlSemiColon);
token!(CtrlColon);
token!(CtrlComma);
token!(CtrlDot);
token!(CtrlLBrace);
token!(CtrlRBrace);
token!(CtrlLBracet);
token!(CtrlRBracet);
token!(CtrlLParan);
token!(CtrlRParan);
token!(CtrlRightArrow);
token!(CtrlThickRightArrow);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ctrl {
    Star(CtrlStar),                       // *
    Slash(CtrlSlash),                     // /
    SemiColon(CtrlSemiColon),             // ;
    Colon(CtrlColon),                     // :
    Comma(CtrlComma),                     // ,
    Dot(CtrlDot),                         // .
    LBrace(CtrlLBrace),                   // {
    RBrace(CtrlRBrace),                   // }
    LBracet(CtrlLBracet),                 // [
    RBracet(CtrlRBracet),                 // ]
    LParan(CtrlLParan),                   // (
    RParan(CtrlRParan),                   // )
    RightArrow(CtrlRightArrow),           // ->
    ThickRightArrow(CtrlThickRightArrow), // =>
}

impl std::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Star(ctrl) => write!(f, "{ctrl}"),
            Self::Slash(ctrl) => write!(f, "{ctrl}"),
            Self::SemiColon(ctrl) => write!(f, "{ctrl}"),
            Self::Colon(ctrl) => write!(f, "{ctrl}"),
            Self::Comma(ctrl) => write!(f, "{ctrl}"),
            Self::Dot(ctrl) => write!(f, "{ctrl}"),
            Self::LBrace(ctrl) => write!(f, "{ctrl}"),
            Self::RBrace(ctrl) => write!(f, "{ctrl}"),
            Self::LBracet(ctrl) => write!(f, "{ctrl}"),
            Self::RBracet(ctrl) => write!(f, "{ctrl}"),
            Self::LParan(ctrl) => write!(f, "{ctrl}"),
            Self::RParan(ctrl) => write!(f, "{ctrl}"),
            Self::RightArrow(ctrl) => write!(f, "{ctrl}"),
            Self::ThickRightArrow(ctrl) => write!(f, "{ctrl}"),
        }
    }
}

from_token!(Ctrl, Star, CtrlStar);
from_token!(Ctrl, Slash, CtrlSlash);
from_token!(Ctrl, SemiColon, CtrlSemiColon);
from_token!(Ctrl, Colon, CtrlColon);
from_token!(Ctrl, Comma, CtrlComma);
from_token!(Ctrl, Dot, CtrlDot);
from_token!(Ctrl, LBrace, CtrlLBrace);
from_token!(Ctrl, RBrace, CtrlRBrace);
from_token!(Ctrl, LBracet, CtrlLBracet);
from_token!(Ctrl, RBracet, CtrlRBracet);
from_token!(Ctrl, LParan, CtrlLParan);
from_token!(Ctrl, RParan, CtrlRParan);
from_token!(Ctrl, RightArrow, CtrlRightArrow);
from_token!(Ctrl, ThickRightArrow, CtrlThickRightArrow);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Statement {
    pub stmt: Expr,
    pub span: Span,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { stmt, .. } = &self;
        write!(f, "({stmt})")
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Type(pub Ident);
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(ident) = &self;
        write!(f, "({ident})")
    }
}

impl From<&Ident> for Type {
    fn from(value: &Ident) -> Self {
        Self(value.clone())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Param {
    pub name: Ident,
    pub kind: Type,
    pub span: Span,
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { name, kind, .. } = &self;
        write!(f, "({name}: {kind})")
    }
}

impl From<(&Ident, &Ident)> for Param {
    fn from((name, kind): (&Ident, &Ident)) -> Self {
        let span = Span::from((name.span, kind.span));
        Self {
            name: name.clone(),
            kind: kind.into(),
            span,
        }
    }
}
