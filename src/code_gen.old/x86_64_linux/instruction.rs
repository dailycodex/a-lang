use super::{mnemonic::Mnemonic, x86reg::X86Reg};
use either::Either;
use std::fmt;

#[derive(Debug, Default)]
pub struct Instruction {
    pub mnemonic: Option<Mnemonic>,
    pub arg1: Option<Either<String, X86Reg>>,
    pub arg2: Option<Either<u64, X86Reg>>,
    pub label: Option<String>,
    pub comment: Option<String>,
}

impl Instruction {
    pub fn mnemonic(mut self, mnemonic: Mnemonic) -> Self {
        self.mnemonic = Some(mnemonic);
        self
    }

    pub fn arg1(mut self, arg1: Either<String, X86Reg>) -> Self {
        self.arg1 = Some(arg1);
        self
    }
    pub fn arg1_label(mut self, arg1: impl Into<String>) -> Self {
        self.arg1(Either::Left(arg1.into()))
    }

    pub fn arg1_reg(mut self, arg1: impl Into<X86Reg>) -> Self {
        self.arg1(Either::Right(arg1.into()))
    }

    pub fn arg2(mut self, arg2: Either<u64, X86Reg>) -> Self {
        self.arg2 = Some(arg2);
        self
    }

    pub fn arg2_value(mut self, arg2: u64) -> Self {
        self.arg2(Either::Left(arg2))
    }

    pub fn arg2_reg(mut self, arg2: impl Into<X86Reg>) -> Self {
        self.arg2(Either::Right(arg2.into()))
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arg2 = self.arg2.map(|i| format!(", {i}")).unwrap_or_default();
        let arg1 = self
            .arg1
            .as_ref()
            .map(|i| match self.mnemonic {
                Some(Mnemonic::Call) => format!(" __{i}__"),
                _ => format!(" {i}"),
            })
            .unwrap_or_default();
        let mnemonic = self
            .mnemonic
            .map(|i| format!("  {i}{arg1}{arg2}"))
            .unwrap_or_default();
        let label = self
            .label
            .as_ref()
            .map(|i| format!("__{i}__:\n"))
            .unwrap_or_default();
        let comment = self
            .comment
            .as_ref()
            .map(|i| format!("; {i}"))
            .unwrap_or_default();
        write!(f, "{label}{mnemonic}{comment}\n")
    }
}
