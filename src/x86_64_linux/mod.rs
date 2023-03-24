mod reg_preserved_64;
mod x86reg;
mod reg64;
mod reg8;
pub use reg_preserved_64::RegPreserved64;
pub use x86reg::X86Reg;
pub use reg64::Reg64;
pub use reg8::Reg8;
use std::fmt;

use crate::ir;
pub fn code_gen(ir: Vec<ir::Instruction>) -> Result<String, Vec<String>> {
    compile_ir_code(ir).and_then(instruction_to_string)
}

fn compile_ir_code(ir: Vec<ir::Instruction>) -> Result<Vec<Instruction>, Vec<String>> {
    let mut state = State::default();
    Ok(ir.iter()
       .map(|i| i.compile(&mut state))
       .collect::<Vec<Instruction>>())
}

fn instruction_to_string(ir: Vec<Instruction>) -> Result<String, Vec<String>> {
       Ok(ir.iter()
       .map(ToString::to_string)
       .collect())
}

#[derive(Debug, Default)]
struct State {
    _i: usize,
}

trait Compile {
    fn compile(&self, state: &mut State) -> Instruction;
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    MoveImm(X86Reg, u64),
    MoveReg(X86Reg, X86Reg),
    Add(X86Reg, X86Reg),
    Sub(X86Reg, X86Reg),
    Mul(X86Reg, X86Reg),
    Div(X86Reg, X86Reg),
    ProLog,
    Epilog,

}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MoveImm(des, value) => write!(f, "mov {des}, {value}\n"),
            Self::MoveReg(des, src) => write!(f, "mov {des}, {src}\n"),
            Self::Add(des, reg) => write!(f, "add {des}, {reg}\n"),
            Self::Sub(des, reg) => write!(f, "sub {des}, {reg}\n"),
            Self::Mul(des, reg) => write!(f, "imul {des}, {reg}\n"),
            Self::Div(des, reg) => write!(f, "idiv {des}, {reg}\n"),
            Self::ProLog => write!(f, "  push rbp\n  mov rbp, rsp\n"),
            Self::Epilog => write!(f, "  mov rsp, rbp\n  pop rbp\n  ret\n"),
        }
    }
}

impl Compile for ir::Instruction {
    fn compile(&self, state: &mut State) -> Instruction {
        match self {
            ir::Instruction::LoadImm(i) => i.compile(state),
            ir::Instruction::LoadVar(i) => i.compile(state),
            ir::Instruction::Add(i) => i.compile(state),
            ir::Instruction::Sub(i) => i.compile(state),
            ir::Instruction::Mul(i) => i.compile(state),
            ir::Instruction::Div(i) => i.compile(state),
            ir::Instruction::Copy(i) => i.compile(state),
            ir::Instruction::Conditional(i) => i.compile(state),
            ir::Instruction::Jump(i) => i.compile(state),
            ir::Instruction::DefLabel(i) => i.compile(state),
            ir::Instruction::Call(i) => i.compile(state),
            ir::Instruction::Enter(i) => i.compile(state),
            ir::Instruction::Leave(i) => i.compile(state),
        }
    }
}

// LoadImm(LoadImm),
impl Compile for ir::LoadImm {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("LoadImm")
    }
}
// LoadVar(LoadVar),
impl Compile for ir::LoadVar {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("LoadVar")
    }
}
// Add(Add),
impl Compile for ir::Add {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("Add")
    }
}
// Sub(Sub),
impl Compile for ir::Sub {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("Sub")
    }
}
// Mul(Mul),
impl Compile for ir::Mul {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("Mul")
    }
}
// Div(Div),
impl Compile for ir::Div {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("Div")
    }
}
// Copy(Copy),
impl Compile for ir::Copy {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("Copy")
    }
}
// Conditional(Conditional),
impl Compile for ir::Conditional {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("Conditional")
    }
}
// Jump(Jump),
impl Compile for ir::Jump {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("Jump")
    }
}
// DefLabel(DefLabel),
impl Compile for ir::DefLabel {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("DefLabel")
    }
}
// Call(Call),
impl Compile for ir::Call {
    fn compile(&self, state: &mut State) -> Instruction {
        unimplemented!("Call")
    }
}
// Enter(Enter),
impl Compile for ir::Enter {
    fn compile(&self, state: &mut State) -> Instruction {
        Instruction::ProLog
    }
}
// Leave(Leave),
impl Compile for ir::Leave {
    fn compile(&self, state: &mut State) -> Instruction {
        Instruction::Epilog
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::lexer::lex;
    use crate::parse::parse;
    use crate::ir;

    fn setup(input: &str) -> Vec<Instruction> {
        lex(input)
            .and_then(parse)
            .and_then(ir::code_gen)
            .and_then(compile_ir_code)
            .unwrap()
    }
    #[test]
    fn basic_test() {
        let left = setup("fn main() { 1 + 2; }");
        let right = vec![
            
        ];
        assert_eq!(left, right);
    }
}
