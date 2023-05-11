use super::{ExprBlock, Ident, Param, Type};
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
    pub keyword_fn: super::keyword::Fn,
    pub name: Ident,
    pub params: Vec<Param>,
    pub block: ExprBlock,
    pub ret_type: Option<Type>,
}

impl ItemFn {
    pub fn new(
        keyword_fn: super::keyword::Fn,
        name: Ident,
        params: Vec<Param>,
        block: ExprBlock,
        ret_type: Option<Type>,
    ) -> Self {
        Self {
            keyword_fn,
            name,
            params,
            block,
            ret_type,
        }
    }

    // pub fn span(&self) -> Span {
    //     let start = self.keyword_fn.span();
    //     let end = self.block.span();
    //     Span::from((start, end))
    // }
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
