use super::{input::Input, label::Label, op::Op, reg::Reg};
#[derive(Debug, PartialEq, Eq)]
pub enum BlockType {
    Assignment(Assignment),
    Copy { des: Reg, from: Input },
    Conditional { x: Input, y: Input, jump: Label },
    Jump { label: Label },
    Label { label: Label },
}

impl BlockType {
    pub fn is_exit(&self) -> bool {
        match self {
            Self::Conditional { .. } | Self::Jump { .. } => true,
            _ => false,
        }
    }
    pub fn is_enter(&self) -> bool {
        match self {
            Self::Label { .. } => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Assignment {
    pub des: Reg,
    pub op: Op,
    pub x: Input,
    pub y: Input,
}
