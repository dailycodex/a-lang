use super::{Block, Ident, Param, Type};
use crate::lexer::{Span, Token};
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Fn(ItemFn),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fn(item_fn) => write!(f, "{item_fn}"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemFn {
    pub name: Ident,
    pub params: Vec<Param>,
    pub block: Block,
    pub ret_type: Option<Type>,
    pub span: Span,
}

impl fmt::Display for ItemFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            params,
            block,
            ret_type,
            ..
        } = &self;
        let ret = ret_type
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or("NULL".into());
        let params = params.iter().map(ToString::to_string).collect::<String>();
        write!(f, "(func {name} <{ret}> ({params}) {block})")
    }
}
