use super::{Mem, X86Reg};
use crate::code_gen::ir::{Reg, Var};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableInput {
    X86Reg(X86Reg),
    IrReg(Reg),
    Var(Var),
}

impl From<&X86Reg> for TableInput {
    fn from(value: &X86Reg) -> Self {
        Self::X86Reg(*value)
    }
}

impl From<X86Reg> for TableInput {
    fn from(value: X86Reg) -> Self {
        Self::X86Reg(value)
    }
}

impl From<&Reg> for TableInput {
    fn from(value: &Reg) -> Self {
        Self::IrReg(*value)
    }
}

impl From<Reg> for TableInput {
    fn from(value: Reg) -> Self {
        Self::IrReg(value)
    }
}

impl From<Var> for TableInput {
    fn from(value: Var) -> Self {
        Self::Var(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableOutput {
    Reg(X86Reg),
    Var(Var),
    Mem(Mem),
}

impl From<X86Reg> for TableOutput {
    fn from(value: X86Reg) -> Self {
        Self::Reg(value)
    }
}

impl From<Var> for TableOutput {
    fn from(value: Var) -> Self {
        Self::Var(value)
    }
}

impl From<Mem> for TableOutput {
    fn from(value: Mem) -> Self {
        Self::Mem(value)
    }
}
