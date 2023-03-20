mod expr;
mod item;
pub mod keyword;
mod lit;

use crate::lexer::Span;
pub use expr::{Expr, ExprBinary, ExprCall, ExprLit, ExprVar};
pub use item::{Item, ItemFn};
pub use lit::{Lit, LitBool, LitChar, LitInt, LitStr};

#[macro_export]
macro_rules! token {
    ($name:ident) => {
        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
pub struct Block {
    pub stmts: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Statement {
    pub stmt: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Type(pub Ident);

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
