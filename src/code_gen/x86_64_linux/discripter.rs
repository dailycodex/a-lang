use super::X86Reg;
use crate::code_gen::ir::{Reg, Var};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Discripter {
    X86Reg(X86Reg),
    IrReg(Reg),
    Var(Var),
}

impl From<&X86Reg> for Discripter {
    fn from(value: &X86Reg) -> Self {
        Self::X86Reg(*value)
    }
}

impl From<X86Reg> for Discripter {
    fn from(value: X86Reg) -> Self {
        Self::X86Reg(value)
    }
}

impl From<&Reg> for Discripter {
    fn from(value: &Reg) -> Self {
        Self::IrReg(*value)
    }
}

impl From<Reg> for Discripter {
    fn from(value: Reg) -> Self {
        Self::IrReg(value)
    }
}
