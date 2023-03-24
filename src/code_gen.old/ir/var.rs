use crate::parse::{Ident, Param};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var(pub String);

impl From<&Ident> for Var {
    fn from(value: &Ident) -> Self {
        Self(value.value.clone())
    }
}

impl From<&Param> for Var {
    fn from(value: &Param) -> Self {
        Self::from(&value.name)
    }
}
