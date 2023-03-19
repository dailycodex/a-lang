use super::{Input, Label, Op, Reg};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IrBlock {
    des: Option<Reg>,
    arg1: Option<Input>,
    arg2: Option<Input>,
    op: Option<Op>,
    label: Option<Label>,
    jump: Option<Label>,
    call: Option<Label>,
}

impl IrBlock {
    pub fn des(mut self, reg: Reg) -> Self {
        self.des = Some(reg);
        self
    }

    pub fn arg1(mut self, input: Input) -> Self {
        self.arg1 = Some(input);
        self
    }

    pub fn arg2(mut self, input: Input) -> Self {
        self.arg2 = Some(input);
        self
    }

    pub fn op(mut self, op: Op) -> Self {
        self.op = Some(op);
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn jump(mut self, label: Label) -> Self {
        self.jump = Some(label);
        self
    }

    pub fn call(mut self, label: Label) -> Self {
        self.call = Some(label);
        self
    }

    pub fn build(self) -> BlockType {
        let Self { des, arg1, arg2, op, label, jump, call} = self;
        if let Some(label) = call {
            return BlockType::Call(label);
        }
        if let Some(label) = jump {
            return BlockType::Call(label);
        }
        match (des, arg1, arg2, op, label) {
            (Some(des), Some(x), Some(y), Some(op), None) => {
                BlockType::Assignment(Assignment { des, op, x, y, })
            },
            (Some(des), Some(from), None, None, None) => {
                BlockType::Copy { des, from }
            }
            (None, None, None, None, Some(label)) => {
                BlockType::Label(label)
            },
            (None, Some(x), Some(y), Some(op), Some(jump)) => {
                BlockType::Conditional(Conditional { jump, op, x, y })
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockType {
    Assignment(Assignment),
    Copy { des: Reg, from: Input },
    Conditional(Conditional),
    Jump(Label),
    Label(Label),
    Call(Label),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    pub des: Reg,
    pub op: Op,
    pub x: Input,
    pub y: Input,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conditional {
    pub jump: Label,
    pub op: Op,
    pub x: Input,
    pub y: Input,
}
