use super::{Imm, Label, Reg, Var};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    LoadImm(LoadImm),
    LoadVar(LoadVar),
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Copy(Copy),
    Conditional(Conditional),
    Jump(Jump),
    DefLabel(DefLabel),
    Call(Call),
    Enter(Enter),
    Leave(Leave),
}

impl Instruction {
    pub fn is_exit(&self) -> bool {
        match self {
            Self::Conditional(..) | Self::Jump(..) => true,
            _ => false,
        }
    }

    pub fn is_enter(&self) -> bool {
        match self {
            Self::DefLabel(..) => true,
            _ => false,
        }
    }
}

macro_rules! from_to {
    ($from:ident, $to:ident) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                Self::$from(value)
            }
        }
    };
}

from_to!(LoadImm, Instruction);
from_to!(LoadVar, Instruction);
from_to!(Copy, Instruction);
from_to!(Conditional, Instruction);
from_to!(Jump, Instruction);
from_to!(DefLabel, Instruction);
from_to!(Call, Instruction);
from_to!(Enter, Instruction);
from_to!(Leave, Instruction);

macro_rules! op_instruction {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            pub des: Reg,
            pub lhs: Reg,
            pub rhs: Reg,
        }

        impl From<$name> for Instruction {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }
    };
}
op_instruction!(Add);
op_instruction!(Sub);
op_instruction!(Mul);
op_instruction!(Div);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadImm {
    pub des: Reg,
    pub imm: Imm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadVar {
    pub des: Reg,
    pub var: Var,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Copy {
    pub to: Reg,
    pub from: Reg, // Either<Var, Reg>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conditional {
    pub jump: Label,
    pub check: Reg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub caller: Label,
    pub args: Vec<Reg>,
    pub ret: Reg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Jump {
    pub label: Label,
}

impl Jump {
    pub fn name(&self) -> String {
        self.label.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefLabel {
    pub label: Label,
}

impl DefLabel {
    pub fn name(&self) -> String {
        self.label.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leave;
