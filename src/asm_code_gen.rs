#![allow(unused)]
use crate::ir_code_gen::*;
use either::Either;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Discripter {
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

#[derive(Debug, Default)]
struct AsmGenerator {
    regs: [bool; 9],
    discripter: HashMap<Discripter, X86Reg>,
    code: Vec<Instruction>,
}

impl AsmGenerator {
    fn return_last_reg_in_exit(&mut self) {
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
            .find(|(i, b)| !*b)
            .map(|(i, _)| i)
            .unwrap();
        self.regs[idx] = true;
        let Some(result) = self.get_reg_from(&Reg(idx)) else {
            unimplemented!()
        };
        result
    }

    fn get_reg_from(&self, reg: &Reg) -> Option<X86Reg> {
        use Reg64::*;
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
            Op::Equal => Mnemonic::SetEqual,
            Op::Grt => Mnemonic::SetGreater,
            Op::Les => Mnemonic::SetLesser,
            Op::Geq => Mnemonic::SetGreaterEqual,
            Op::Leq => Mnemonic::SetLesserEqual,
            Op::Neq => Mnemonic::SetNotEqual,
            _ => unimplemented!(),
        };
        self.instruction(mnemonic, arg1.lower_8bit(), None);
        self.instruction(
            Mnemonic::MoveZx,
            arg1.into(),
            Some(Either::Right(arg1.lower_8bit().into())),
        );
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

    fn compile(&mut self, basic_block: &BlockType) {
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

pub fn code_gen(blocks: &[BasicBlock]) -> String {
    let mut asm_generator = AsmGenerator::default();
    for bb in blocks.iter() {
        for b in bb.blocks.iter() {
            asm_generator.compile(b)
        }
    }
    asm_generator.return_last_reg_in_exit();
    // dbg!(&asm_generator);
    // let asm = blocks
    //     .iter()
    //     .map(|bs| {
    //         bs.blocks
    //             .iter()
    //             .map(|b| asm_generator.compile(b).to_string())
    //             .collect::<String>()
    //     })
    //     .collect::<String>();
    format!(
        "
format ELF64 executable 3
segment readable executable

entry _start

_start:
{asm_generator}
  mov rax, 60
  syscall

segment readable writable
"
    )
}

#[derive(Debug)]
struct Instruction {
    mnemonic: Mnemonic,
    arg1: X86Reg,
    arg2: Option<Either<u64, X86Reg>>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arg2 = self.arg2.map(|i| format!(", {i}")).unwrap_or("".into());
        write!(f, "{} {}{}\n", self.mnemonic, self.arg1, arg2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mnemonic {
    Move,
    MoveZx,
    Add,
    Sub,
    Mul,
    Div,
    Cmp,
    SetEqual,
    SetGreater,
    SetLesser,
    SetGreaterEqual,
    SetLesserEqual,
    SetNotEqual,
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Move => write!(f, "mov"),
            Self::MoveZx => write!(f, "movzx"),
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Mul => write!(f, "imul"),
            Self::Div => write!(f, "idiv"),
            Self::Cmp => write!(f, "cmp"),
            Self::SetEqual => write!(f, "sete"),
            Self::SetGreater => write!(f, "setg"),
            Self::SetLesser => write!(f, "setl"),
            Self::SetGreaterEqual => write!(f, "setge"),
            Self::SetLesserEqual => write!(f, "setle"),
            Self::SetNotEqual => write!(f, "setne"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum X86Reg {
    Reg64(Reg64),
    Reg8(Reg8),
}

impl X86Reg {
    fn lower_8bit(&self) -> Self {
        match self {
            Self::Reg64(reg) => Self::Reg8(reg.lower_8bit()),
            _ => *self,
        }
    }
}

impl X86Reg {
    fn as_usize(&self) -> usize {
        match self {
            X86Reg::Reg64(r) => *r as usize,
            X86Reg::Reg8(r) => *r as usize,
        }
    }
}
impl From<Reg64> for X86Reg {
    fn from(value: Reg64) -> Self {
        Self::Reg64(value)
    }
}

impl From<Reg8> for X86Reg {
    fn from(value: Reg8) -> Self {
        Self::Reg8(value)
    }
}

impl fmt::Display for X86Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reg64(reg) => write!(f, "{reg}"),
            Self::Reg8(reg) => write!(f, "{reg}"),
        }
    }
}

#[repr(usize)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Reg64 {
    Rax = 0,
    Rdi,
    Rsi,
    Rdx,
    Rcx,
    R8,
    R9,
    R10,
    R11,
}

impl Reg64 {
    fn lower_8bit(&self) -> Reg8 {
        match self {
            Self::Rax => Reg8::Al,
            Self::Rdi => Reg8::Dil,
            Self::Rsi => Reg8::Sil,
            Self::Rdx => Reg8::Dl,
            Self::Rcx => Reg8::Cl,
            Self::R8 => Reg8::R8b,
            Self::R9 => Reg8::R9b,
            Self::R10 => Reg8::R10b,
            Self::R11 => Reg8::R11b,
        }
    }
}

impl fmt::Display for Reg64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rax => write!(f, "rax"),
            Self::Rdi => write!(f, "rdi"),
            Self::Rsi => write!(f, "rsi"),
            Self::Rdx => write!(f, "rdx"),
            Self::Rcx => write!(f, "rcx"),
            Self::R8 => write!(f, "r8"),
            Self::R9 => write!(f, "r9"),
            Self::R10 => write!(f, "r10"),
            Self::R11 => write!(f, "r11"),
        }
    }
}

#[repr(usize)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Reg8 {
    Al = 0,
    Dil,
    Sil,
    Dl,
    Cl,
    R8b,
    R9b,
    R10b,
    R11b,
}

impl fmt::Display for Reg8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Al => write!(f, "al"),
            Self::Dil => write!(f, "ril"),
            Self::Sil => write!(f, "sil"),
            Self::Dl => write!(f, "dl"),
            Self::Cl => write!(f, "cl"),
            Self::R8b => write!(f, "r8b"),
            Self::R9b => write!(f, "r9b"),
            Self::R10b => write!(f, "r10b"),
            Self::R11b => write!(f, "r11b"),
        }
    }
}
