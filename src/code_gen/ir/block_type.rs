use super::{Input, Label, Op, Reg};

// #[derive(Debug, Default, Clone, PartialEq, Eq)]
// pub struct IrBlock {
//     des: Option<Reg>,
//     arg1: Option<Input>,
//     arg2: Option<Input>,
//     op: Option<Op>,
//     label: Option<Label>,
//     jump: Option<Label>,
//     call: Option<Label>,
// }
//
// impl IrBlock {
//     pub fn des(mut self, reg: Reg) -> Self {
//         self.des = Some(reg);
//         self
//     }
//
//     pub fn arg1(mut self, input: Input) -> Self {
//         self.arg1 = Some(input);
//         self
//     }
//
//     pub fn arg2(mut self, input: Input) -> Self {
//         self.arg2 = Some(input);
//         self
//     }
//
//     pub fn op(mut self, op: Op) -> Self {
//         self.op = Some(op);
//         self
//     }
//
//     pub fn label(mut self, label: Label) -> Self {
//         self.label = Some(label);
//         self
//     }
//
//     pub fn jump(mut self, label: Label) -> Self {
//         self.jump = Some(label);
//         self
//     }
//
//     pub fn call(mut self, label: Label) -> Self {
//         self.call = Some(label);
//         self
//     }
//
//     pub fn build(self) -> BlockType {
//         let Self { des, arg1, arg2, op, label, jump, call} = self;
//         if let Some(label) = call {
//             return BlockType::Call(label);
//         }
//         if let Some(label) = jump {
//             return BlockType::Call(label);
//         }
//         match (des, arg1, arg2, op, label) {
//             (Some(des), Some(x), Some(y), Some(op), None) => {
//                 BlockType::Assignment(Assignment { des, op, x, y, })
//             },
//             (Some(to), Some(from), None, None, None) => {
//                 BlockType::Copy(IrCopy{ to, from })
//             }
//             (None, None, None, None, Some(label)) => {
//                 BlockType::Label(label)
//             },
//             (None, Some(x), Some(y), Some(op), Some(jump)) => {
//                 BlockType::Conditional(Conditional { jump, op, x, y })
//             },
//             _ => unreachable!(),
//         }
//     }
// }
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockType {
    Assignment(Assignment),
    Copy(IrCopy),
    Conditional(Conditional),
    Jump(Jump),
    Label(DefLabel),
    Call(Call),
    Enter,
    Leave,
}

impl BlockType {
    pub fn is_exit(&self) -> bool {
        match self {
            Self::Conditional(..) | Self::Jump(..) => true,
            _ => false,
        }
    }

    pub fn is_enter(&self) -> bool {
        match self {
            Self::Label(..) => true,
            _ => false,
        }
    }
}

impl From<Assignment> for BlockType {
    fn from(value: Assignment) -> Self {
        Self::Assignment(value)
    }
}

impl From<IrCopy> for BlockType {
    fn from(value: IrCopy) -> Self {
        Self::Copy(value)
    }
}

impl From<Conditional> for BlockType {
    fn from(value: Conditional) -> Self {
        Self::Conditional(value)
    }
}

impl From<Jump> for BlockType {
    fn from(value: Jump) -> Self {
        Self::Jump(value)
    }
}

impl From<DefLabel> for BlockType {
    fn from(value: DefLabel) -> Self {
        Self::Label(value)
    }
}

impl From<Enter> for BlockType {
    fn from(value: Enter) -> Self {
        Self::Enter
    }
}

impl From<Leave> for BlockType {
    fn from(value: Leave) -> Self {
        Self::Leave
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub des: Reg,
    pub op: Op,
    pub lhs: Input,
    pub rhs: Input,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrCopy {
    pub to: Reg,
    pub from: Input,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conditional {
    pub jump: Label,
    pub op: Op,
    pub x: Input,
    pub y: Input,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub caller: Label,
    pub args: Vec<Input>,
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
