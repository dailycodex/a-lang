use super::{
    mnemonic::Mnemonic,
    x86reg::X86Reg,
};
use either::Either;
use std::fmt;

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub arg1: X86Reg,
    pub arg2: Option<Either<u64, X86Reg>>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arg2 = self.arg2.map(|i| format!(", {i}")).unwrap_or("".into());
        write!(f, "{} {}{}\n", self.mnemonic, self.arg1, arg2)
    }
}
