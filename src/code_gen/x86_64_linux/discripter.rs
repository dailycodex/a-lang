use crate::code_gen::ir::Reg;
use super::X86Reg;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Discripter {
    X86Reg(X86Reg),
    IrReg(Reg),
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
