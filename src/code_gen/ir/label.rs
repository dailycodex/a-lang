use crate::parse::Name;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

impl From<&Name> for Label {
    fn from(value: &Name) -> Self {
        Self(value.name.clone())
    }
}
