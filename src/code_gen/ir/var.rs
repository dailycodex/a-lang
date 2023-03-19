use crate::parse::{Name, Param};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var(pub String);

impl From<&Name> for Var {
    fn from(value: &Name) -> Self {
        Self(value.name.clone())
    }
}

impl From<&Param> for Var {
    fn from(value: &Param) -> Self {
        Self(value.name.clone())
    }
}
