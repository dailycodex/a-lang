use super::{Reg, Value};
#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    Reg(Reg),
    Value(Value),
}
