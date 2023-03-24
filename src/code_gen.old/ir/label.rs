use crate::parse::Ident;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

impl From<&Ident> for Label {
    fn from(value: &Ident) -> Self {
        Self(value.value.clone())
    }
}
