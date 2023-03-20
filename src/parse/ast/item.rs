use super::{Block, Ident, Param, Type};
use crate::lexer::{Span, Token};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Fn(ItemFn),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemFn {
    pub name: Ident,
    pub params: Vec<Param>,
    pub block: Block,
    pub ret_type: Option<Type>,
    pub span: Span,
}
