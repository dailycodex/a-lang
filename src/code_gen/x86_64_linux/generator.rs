use super::{
    X86Reg,
    Reg64,
    Discripter,
    Instruction,
    Mnemonic,
};
use crate::code_gen::ir::{
    Op,
    BlockType,
    Input,
    Value,
    Reg,
    Assignment,
};
use either::Either;
use std::{collections::HashMap, fmt};

#[derive(Debug, Default)]
pub struct AsmGenerator {
    regs: [bool; 9],
    discripter: HashMap<Discripter, X86Reg>,
    code: Vec<Instruction>,
}

impl AsmGenerator {
    pub fn return_last_reg_in_exit(&mut self) {
        if let Some((_, xreg)) = self.discripter.iter().max() {
            if *xreg != X86Reg::Reg64(Reg64::Rdi) {
                self.code.push(Instruction {
                    mnemonic: Mnemonic::Move,
                    arg1: Reg64::Rdi.into(),
                    arg2: Some(Either::Right(*xreg)),
                });
            }
        }
    }

    fn get_reg(&mut self) -> X86Reg {
        let idx = self
            .regs
            .iter()
            .enumerate()
            .find(|(_, b)| !*b)
            .map(|(i, _)| i)
            .unwrap();
        self.regs[idx] = true;
        let Some(result) = self.get_reg_from(&Reg(idx)) else {
            unimplemented!()
        };
        result
    }

    fn get_reg_from(&self, reg: &Reg) -> Option<X86Reg> {
        use super::Reg64::*;
        match reg.0 {
            0 => Some(Rax.into()),
            1 => Some(Rdi.into()),
            2 => Some(Rsi.into()),
            3 => Some(Rdx.into()),
            4 => Some(Rcx.into()),
            5 => Some(R8.into()),
            6 => Some(R9.into()),
            7 => Some(R10.into()),
            8 => Some(R11.into()),
            _ => None,
        }
    }

    fn regester_reg(&mut self, xreg: X86Reg, reg: &Reg) -> Either<u64, X86Reg> {
        self.discripter.insert((*reg).into(), xreg);
        Either::Right(xreg)
    }

    fn release_reg(&mut self, name: X86Reg) {
        let idx = name.as_usize();
        self.regs[idx] = false;
        self.discripter.remove(&name.into());
    }

    fn mov(&mut self, value: String) -> Either<u64, X86Reg> {
        let des = self.get_reg();
        let num = value.parse::<u64>().unwrap();
        self.code.push(Instruction {
            mnemonic: Mnemonic::Move,
            arg1: des,
            arg2: Some(Either::Left(num)),
        });
        Either::Right(des.into())
    }

    fn op_instruction(&mut self, op: &Op, arg1: X86Reg) {
        let mnemonic = match op {
            Op::Equal => Some(Mnemonic::SetEqual),
            Op::Grt => Some(Mnemonic::SetGreater),
            Op::Les => Some(Mnemonic::SetLesser),
            Op::Geq => Some(Mnemonic::SetGreaterEqual),
            Op::Leq => Some(Mnemonic::SetLesserEqual),
            Op::Neq => Some(Mnemonic::SetNotEqual),
            _ => None,
        };
        if let Some(mnemonic) = mnemonic {
            self.instruction(mnemonic, arg1.lower_8bit(), None);
            self.instruction(
                Mnemonic::MoveZx,
                arg1.into(),
                Some(Either::Right(arg1.lower_8bit().into())),
            );
        }
    }

    fn instruction(&mut self, mnemonic: Mnemonic, arg1: X86Reg, arg2: Option<Either<u64, X86Reg>>) {
        self.code.push(Instruction {
            mnemonic,
            arg1,
            arg2,
        });
    }

    fn visit_op(&mut self, op: &Op) -> Mnemonic {
        match op {
            Op::Add => Mnemonic::Add,
            Op::Sub => Mnemonic::Sub,
            Op::Mult => Mnemonic::Mul,
            Op::Div => Mnemonic::Div,
            Op::Equal => Mnemonic::Cmp,
            Op::Grt => Mnemonic::Cmp,
            Op::Les => Mnemonic::Cmp,
            Op::Geq => Mnemonic::Cmp,
            Op::Leq => Mnemonic::Cmp,
            Op::Neq => Mnemonic::Cmp,
        }
    }

    fn visit_reg(&mut self, reg: &Reg) -> X86Reg {
        self.discripter.get(&reg.into()).cloned().unwrap_or({
            let xreg = self.get_reg();
            self.regester_reg(xreg, reg);
            xreg
        })
    }

    fn visit_value(&mut self, value: &Value) -> String {
        let Value(value) = value;
        value.to_string()
    }

    fn visit_input(&mut self, input: &Input) -> Either<String, X86Reg> {
        match input {
            Input::Reg(reg) => Either::Right(self.visit_reg(reg)),
            Input::Value(value) => Either::Left(self.visit_value(value)),
        }
    }

    fn visit_assignment(&mut self, block_type: &Assignment) {
        let Assignment { des, op, x, y } = &block_type;
        let mnemonic = self.visit_op(op);
        let arg1 = self
            .visit_input(x)
            .left_and_then(|x| self.mov(x))
            .right_and_then(|x86reg| self.regester_reg(x86reg, des))
            .right()
            .unwrap();
        let arg2 =
            Some(self.visit_input(y).left_and_then(|string| {
                Either::<u64, X86Reg>::Left(string.parse::<u64>().unwrap())
            }));
        self.instruction(mnemonic, arg1.into(), arg2);
        self.op_instruction(op, arg1);
        // self.release_reg(arg1);
        // arg2.map(|x| x.right_and_then(|x| {
        //     self.release_reg(x);
        //     Either::Right(x)
        // }));
    }

    pub fn compile(&mut self, basic_block: &BlockType) {
        match basic_block {
            BlockType::Assignment(assignment) => self.visit_assignment(assignment),
            BlockType::Copy { .. } => unimplemented!(),
            BlockType::Conditional { .. } => unimplemented!(),
            BlockType::Jump { .. } => unimplemented!(),
            BlockType::Label { .. } => unimplemented!(),
        }
    }
}

impl fmt::Display for AsmGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .code
            .iter()
            .map(ToString::to_string)
            .collect::<String>();
        write!(f, "{string}")
    }
}
