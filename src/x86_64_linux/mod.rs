mod reg_state;
#[cfg(test)]
mod test;
pub mod x86reg;
use reg_state::RegState;
pub use std::fmt;
pub use x86reg::*;

use crate::ir;
// pub fn code_gen(ir: Vec<ir::Instruction>) -> Result<String, Vec<String>> {
//     compile_ir_code(ir).and_then(instruction_to_string)
// }

pub fn compile_ir_code(ir: Vec<ir::Instruction>) -> Result<Vec<Instruction>, Vec<String>> {
    let mut state = RegState::default();
    Ok(ir
        .iter()
        .map(|i| i.compile(&mut state))
        .collect::<Vec<Vec<Instruction>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<Instruction>>())
}

pub fn instruction_to_string(ir: Vec<Instruction>) -> Result<String, Vec<String>> {
    Ok(ir.iter().map(ToString::to_string).collect())
}

trait Compile {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    MoveImm(X86Reg, u64),
    MoveReg(X86Reg, X86Reg),
    MoveZx(X86Reg),
    Add(X86Reg, X86Reg),
    Sub(X86Reg, X86Reg),
    Mul(X86Reg, X86Reg),
    Div(X86Reg, X86Reg),
    DefLabel(String),
    Call(String),
    Jump(String),
    JumpZero(String),
    Cmp(X86Reg, X86Reg),
    Test(X86Reg, X86Reg),
    SetG,
    ProLog,
    Epilog,
    Syscall,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MoveImm(des, value) => {
                writeln!(f, "{:>10}{:>10},{:>10}", "mov", des.to_string(), value)
            }
            Self::MoveReg(des, src) => writeln!(
                f,
                "{:>10}{:>10},{:>10}",
                "mov",
                des.to_string(),
                src.to_string()
            ),
            Self::MoveZx(src) => writeln!(f, "{:>10}{:>10},{:>10}", "movzx", src.to_string(), "al"),
            Self::Add(des, reg) => writeln!(
                f,
                "{:>10}{:>10},{:>10}",
                "add",
                des.to_string(),
                reg.to_string()
            ),
            Self::Sub(des, reg) => writeln!(
                f,
                "{:>10}{:>10},{:>10}",
                "sub",
                des.to_string(),
                reg.to_string()
            ),
            Self::Mul(des, reg) => writeln!(
                f,
                "{:>10}{:>10},{:>10}",
                "imul",
                des.to_string(),
                reg.to_string()
            ),
            Self::Div(des, reg) => writeln!(
                f,
                "{:>10}{:>10},{:>10}",
                "idiv",
                des.to_string(),
                reg.to_string()
            ),
            Self::DefLabel(name) => writeln!(f, "{}__:", name),
            Self::Call(name) => writeln!(f, "{:>10}{:>10}__", "call", name),
            Self::Jump(name) => writeln!(f, "{:>10}{:>10}__", "jmp", name),
            Self::JumpZero(name) => writeln!(f, "{:>10}{:>10}__", "jz", name),
            Self::Cmp(lhs, rhs) => writeln!(
                f,
                "{:>10}{:>10},{:>10}",
                "cmp",
                lhs.to_string(),
                rhs.to_string()
            ),
            Self::Test(lhs, rhs) => writeln!(
                f,
                "{:>10}{:>10},{:>10}",
                "test",
                lhs.to_string(),
                rhs.to_string()
            ),
            Self::SetG => writeln!(f, "{:>10}{:>10}", "setg", "al"),
            Self::ProLog => {
                let push = format!("{:>10}{:>10}", "push", "rbp");
                let mov = format!("{:>10}{:>10},{:>10}", "mov", "rbp", "rsp");
                writeln!(f, "{push}\n{mov}")
            }
            Self::Epilog => {
                let mov = format!("{:>10}{:>10},{:>10}", "mov", "rbp", "rsp");
                let pop = format!("{:>10}{:>10}", "pop", "rbp");
                let ret = format!("{:>10}", "ret");
                writeln!(f, "{mov}\n{pop}\n{ret}")
            }
            Self::Syscall => writeln!(f, "{:>10}", "syscall"),
        }
    }
}

impl Compile for ir::Instruction {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        match self {
            ir::Instruction::LoadImm(i) => i.compile(state),
            ir::Instruction::DefFunc(i) => i.compile(state),
            ir::Instruction::Add(i) => i.compile(state),
            ir::Instruction::Sub(i) => i.compile(state),
            ir::Instruction::Mul(i) => i.compile(state),
            ir::Instruction::Div(i) => i.compile(state),
            ir::Instruction::Grt(i) => i.compile(state),
            ir::Instruction::Copy(i) => i.compile(state),
            ir::Instruction::Conditional(i) => i.compile(state),
            ir::Instruction::Jump(i) => i.compile(state),
            ir::Instruction::DefLabel(i) => i.compile(state),
            ir::Instruction::Call(i) => i.compile(state),
            ir::Instruction::Return(i) => i.compile(state),
            ir::Instruction::Enter(i) => i.compile(state),
            ir::Instruction::Leave(i) => i.compile(state),
        }
    }
}

// LoadImm(LoadImm),
impl Compile for ir::LoadImm {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let ir::LoadImm {
            des,
            imm: ir::Imm(imm),
        } = self;
        let reg = state.get_reg(des);
        vec![Instruction::MoveImm(reg, *imm)]
    }
}
// DefFunc(DefFunc),
impl Compile for ir::DefFunc {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let ir::DefFunc {
            name,
            ret: _,
            params: _,
            body,
            ..
        } = self;
        // FIXME:getting regesters for the params is currently not implemented correctly.
        let mut result = body
            .iter()
            .flat_map(|inst| inst.compile(state))
            .collect::<Vec<Instruction>>();
        result.insert(0, Instruction::DefLabel(name.into()));
        // let ret_reg = state.get_ret_reg();
        // let last_reg = state.last_used_reg();
        // let instruction = Instruction::MoveReg(ret_reg, last_reg);
        // result.insert(result.len().saturating_sub(1), instruction);
        state.reset();
        result
    }
}
// Add(Add),
impl Compile for ir::Add {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let ir::Add { des, lhs, rhs } = self;
        let xdes = state.get_reg(des);
        let xlhs = state.get_reg(lhs);
        state.release_reg(lhs);
        let xrhs = state.get_reg(rhs);
        state.release_reg(rhs);
        vec![
            Instruction::MoveReg(xdes, xlhs),
            Instruction::Add(xdes, xrhs),
        ]
    }
}
// Sub(Sub),
impl Compile for ir::Sub {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let ir::Sub { des, lhs, rhs } = self;
        let des = state.get_reg(des);
        let lhs = state.get_reg(lhs);
        let rhs = state.get_reg(rhs);
        vec![Instruction::MoveReg(des, lhs), Instruction::Sub(des, rhs)]
    }
}
// Mul(Mul),
impl Compile for ir::Mul {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let ir::Mul { des, lhs, rhs } = self;
        let xdes = state.get_reg(des);
        let xlhs = state.get_reg(lhs);
        state.release_reg(lhs);
        let xrhs = state.get_reg(rhs);
        state.release_reg(rhs);
        vec![
            Instruction::MoveReg(xdes, xlhs),
            Instruction::Mul(xdes, xrhs),
        ]
    }
}
// Div(Div),
impl Compile for ir::Div {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let ir::Div { des, lhs, rhs } = self;
        let des = state.get_reg(des);
        let lhs = state.get_reg(lhs);
        let rhs = state.get_reg(rhs);
        vec![Instruction::MoveReg(des, lhs), Instruction::Div(des, rhs)]
    }
}

impl Compile for ir::Grt {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let ir::Grt { des, lhs, rhs } = self;
        let des = state.get_reg(des);
        let lhs = state.get_reg(lhs);
        let rhs = state.get_reg(rhs);
        vec![
            Instruction::MoveReg(des, lhs),
            Instruction::Cmp(des, rhs),
            Instruction::SetG,
            Instruction::MoveZx(des),
        ]
    }
}

impl Compile for ir::Copy {
    fn compile(&self, _state: &mut RegState) -> Vec<Instruction> {
        unimplemented!("{:?}", self)
    }
}
// Conditional(Conditional),
impl Compile for ir::Conditional {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let des = state.get_reg(&self.reg);
        vec![
            Instruction::Test(des, des),
            Instruction::JumpZero(self.label.to_string()),
        ]
    }
}
// Jump(Jump),
impl Compile for ir::Jump {
    fn compile(&self, _state: &mut RegState) -> Vec<Instruction> {
        vec![Instruction::Jump(self.name())]
    }
}
// DefLabel(DefLabel),
impl Compile for ir::DefLabel {
    fn compile(&self, _state: &mut RegState) -> Vec<Instruction> {
        vec![Instruction::DefLabel(self.name())]
    }
}
// Call(Call),
impl Compile for ir::Call {
    fn compile(&self, _state: &mut RegState) -> Vec<Instruction> {
        vec![Instruction::Call(self.caller.0.to_string())]
    }
}

// Return(Return),
impl Compile for ir::Return {
    fn compile(&self, state: &mut RegState) -> Vec<Instruction> {
        let reg = state.get_reg(&self.0);
        let ret = state.get_ret_reg();
        vec![Instruction::MoveReg(ret, reg)]
    }
}

// Enter(Enter),
impl Compile for ir::Enter {
    fn compile(&self, _state: &mut RegState) -> Vec<Instruction> {
        vec![Instruction::ProLog]
    }
}
// Leave(Leave),
impl Compile for ir::Leave {
    fn compile(&self, _state: &mut RegState) -> Vec<Instruction> {
        vec![Instruction::Epilog]
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::ir;
    // use crate::lexer::lex;
    // use crate::parse::parse;
    // use pretty_assertions::assert_eq;

    // fn setup(input: &str) -> Vec<Instruction> {
    //     lex(input)
    //         .and_then(parse)
    //         .and_then(ir::code_gen)
    //         .and_then(compile_ir_code)
    //         .unwrap()
    // }
    // #[test]
    // fn basic_test() {
    //     let left = setup("fn main() { 1 + 2; }");
    //     let right = vec![];
    //     assert_eq!(left, right);
    // }
}
