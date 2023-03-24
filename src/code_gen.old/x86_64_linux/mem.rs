use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mem(pub usize);

impl From<usize> for Mem {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl fmt::Display for Mem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "qword [rsp-{}]", self.0)
    }
}
