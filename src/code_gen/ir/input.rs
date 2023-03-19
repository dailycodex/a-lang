use super::{Reg, Value, Var};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Reg(Reg),
    Value(Value),
    Var(Var),
}
