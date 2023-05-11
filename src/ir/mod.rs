mod instruction;
#[cfg(test)]
mod test;
use std::collections::HashMap;

pub use instruction::*;

use crate::lexer::*;

use crate::parse::{
    Expr, ExprBinary, ExprBlock, ExprCall, ExprIf, ExprLit, ExprReturn, ExprVar, Ident, Item,
    ItemFn, Lit, LitBool, LitInt, Op, Param, Statement,
};

pub fn code_gen(ast: Vec<Item>) -> Result<Vec<Instruction>, Vec<String>> {
    let mut gen = IrGenerator::default();
    gen.visit(&ast);
    Ok(gen.code)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&Ident> for Label {
    fn from(value: &Ident) -> Self {
        Label(value.value())
    }
}

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        Label(value.to_string())
    }
}

impl From<String> for Label {
    fn from(value: String) -> Self {
        Label(value)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Reg(pub usize);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Var(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imm(pub u64);
impl From<u64> for Imm {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

trait Ir {
    fn def_label(&mut self, label: Label);
    fn jump(&mut self, label: Label);
    fn load_imm(&mut self, imm: Imm) -> Reg;
    fn binary(&mut self, op: &Op, lhs: Reg, rhs: Reg) -> Reg;
    fn conditional(&mut self, label: Label, reg: Reg) -> Reg;
    fn call(&mut self, label: Label, args: Vec<Reg>, ret: Reg) -> Reg;
    fn early_return(&mut self, reg: Reg) -> Reg;
}

trait AstVisitor: Ir {
    fn visit_expr_var(&mut self, expr_var: &ExprVar) -> Reg;
    // FIXME: Not really what i wanted to do.
    fn visit_params(&mut self, expr: &Param) -> Reg;
    fn visit_expr_call(&mut self, expr_call: &ExprCall) -> Reg;
    fn visit_expr_binary(&mut self, bin: &ExprBinary) -> Reg;
    fn visit_item_fn(&mut self, item_fn: &ItemFn);
    fn visit_lit_int(&mut self, lit_int: &LitInt) -> Reg;
    fn visit_lit_bool(&mut self, lit_bool: &LitBool) -> Reg;

    fn visit_expr_if(&mut self, expr_if: &ExprIf) -> Reg;

    fn visit_expr_return(&mut self, expr_ret: &ExprReturn) -> Reg {
        let ExprReturn { expr, .. } = expr_ret;
        let reg = self.visit_expr(expr);
        self.early_return(reg);
        self.jump(".exit".into());
        reg
    }

    fn visit_lit(&mut self, lit: &Lit) -> Reg {
        match lit {
            Lit::Int(ref lint) => self.visit_lit_int(lint),
            Lit::Bool(ref lbool) => self.visit_lit_bool(lbool),
            Lit::Str(_lstr) => unimplemented!(),
            Lit::Char(_lchar) => unimplemented!(),
        }
    }

    fn visit_expr_lit(&mut self, expr_lit: &ExprLit) -> Reg {
        let ExprLit { lit, .. } = &expr_lit;
        self.visit_lit(lit)
    }

    fn visit_expr(&mut self, expr: &Expr) -> Reg {
        match expr {
            Expr::Lit(ref elit) => self.visit_expr_lit(elit),
            Expr::Binary(ref ebinary) => self.visit_expr_binary(ebinary),
            Expr::Call(ref ecall) => self.visit_expr_call(ecall),
            Expr::Var(evar) => self.visit_expr_var(evar),
            Expr::If(eif) => self.visit_expr_if(eif),
            Expr::Block(eblock) => self.visit_expr_block(eblock),
            Expr::Return(ereturn) => self.visit_expr_return(ereturn),
        }
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> Reg {
        let Statement { stmt, .. } = stmt;
        self.visit_expr(stmt)
    }

    fn visit_expr_block(&mut self, block: &ExprBlock) -> Reg {
        // FIXME: This should return a Reg if we keep the current pattern
        let mut reg: Option<Reg> = None;
        for stmt in block.stmts.iter() {
            reg = Some(self.visit_stmt(stmt));
        }
        let Some(reg) = reg else {
            panic!("WHAT DO I DO HERE!");
        };
        reg
    }

    fn visit(&mut self, items: &[Item]) {
        for item in items.iter() {
            match item {
                Item::Fn(ref item_fn) => self.visit_item_fn(item_fn),
            }
        }
    }
}

#[derive(Debug, Default)]
struct IrGenerator {
    code: Vec<Instruction>,
    block: Vec<Instruction>,
    reg_counter: usize,
    vars: HashMap<String, Reg>,
    gen_label_number: usize,
}

impl IrGenerator {
    fn push_to_block(&mut self, ir: impl Into<Instruction>) {
        self.block.push(ir.into());
    }

    fn push_fn(&mut self, ir: impl Into<Instruction>) {
        self.code.push(ir.into());
    }

    fn get_reg(&mut self) -> Reg {
        let reg = Reg(self.reg_counter);
        self.reg_counter += 1;
        reg
    }

    fn reset_regester_count(&mut self) {
        self.reg_counter = 0;
    }

    fn gen_label(&mut self) -> Label {
        let number = self.gen_label_number;
        self.gen_label_number += 1;
        Label(format!(".L{}", number))
    }
}

impl Ir for IrGenerator {
    fn def_label(&mut self, label: Label) {
        let instruction: Instruction = DefLabel(label).into();
        self.push_to_block(instruction);
    }
    fn jump(&mut self, label: Label) {
        let instruction: Instruction = Jump(label).into();
        self.push_to_block(instruction);
    }

    fn load_imm(&mut self, imm: Imm) -> Reg {
        let des = self.get_reg();
        let load = LoadImm { des, imm };
        self.push_to_block(load);
        des
    }

    fn binary(&mut self, op: &Op, lhs: Reg, rhs: Reg) -> Reg {
        let des = self.get_reg();
        let instruction: Instruction = match op {
            Op::Add(_) => Add { des, lhs, rhs }.into(),
            Op::Sub(_) => Sub { des, lhs, rhs }.into(),
            Op::Mul(_) => Mul { des, lhs, rhs }.into(),
            Op::Div(_) => Div { des, lhs, rhs }.into(),
            Op::Grt(_) => Grt { des, lhs, rhs }.into(),
            _ => unimplemented!("{op:?}"),
        };
        self.push_to_block(instruction);
        des
    }

    fn conditional(&mut self, label: Label, reg: Reg) -> Reg {
        let instruction: Instruction = Conditional { reg, label }.into();
        self.push_to_block(instruction);
        reg
    }

    fn call(&mut self, caller: Label, args: Vec<Reg>, ret: Reg) -> Reg {
        let instruction: Instruction = Call { caller, args, ret }.into();
        self.push_to_block(instruction);
        ret
    }
    fn early_return(&mut self, reg: Reg) -> Reg {
        let instruction: Instruction = Return(reg).into();
        self.push_to_block(instruction);
        reg
    }
}

impl AstVisitor for IrGenerator {
    fn visit_expr_var(&mut self, expr_var: &ExprVar) -> Reg {
        let ExprVar { name, .. } = expr_var;
        *self.vars.get(&name.value()).unwrap()
    }

    fn visit_params(&mut self, params: &Param) -> Reg {
        let Param { name, .. } = params;
        let des = self.get_reg();
        self.vars.insert(name.value(), des);
        des
    }

    fn visit_expr_call(&mut self, expr_call: &ExprCall) -> Reg {
        let ExprCall { caller, args, .. } = expr_call;
        let Expr::Var(ExprVar { name, .. }) = &**caller else {
            panic!("expected Ident");
        };
        // FIXME: this reg needs to be stored with var in discriper?
        let ret = Reg(1000);
        let args = args
            .iter()
            .map(|expr| self.visit_expr(expr))
            .collect::<Vec<Reg>>();
        self.call(name.into(), args, ret)
    }

    fn visit_expr_binary(&mut self, bin: &ExprBinary) -> Reg {
        let ExprBinary {
            left, right, op, ..
        } = bin;
        let lhs = self.visit_expr(left);
        let rhs = self.visit_expr(right);
        self.binary(op, lhs, rhs)
    }

    fn visit_item_fn(&mut self, item_fn: &ItemFn) {
        let ItemFn {
            name,
            params,
            block,
            ret_type: _,
            ..
        } = item_fn;

        self.gen_label_number = 0;
        self.reset_regester_count();
        let params = params
            .iter()
            .map(|p| (self.visit_params(p), Type::I64))
            .collect();

        self.push_to_block(Enter);
        self.visit_expr_block(block);
        self.def_label(".exit".into());
        self.push_to_block(Leave);

        let body = self.block.clone();
        self.block.clear();
        self.push_fn(DefFunc {
            name: name.value(),
            params,
            ret: Type::I64,
            body,
        });
    }

    fn visit_lit_int(&mut self, lit_int: &LitInt) -> Reg {
        let imm: Imm = lit_int.parse::<u64>().unwrap().into();
        self.load_imm(imm)
    }

    fn visit_lit_bool(&mut self, lit_bool: &LitBool) -> Reg {
        let num: bool = lit_bool.parse::<bool>().unwrap();
        let imm: Imm = (num as u64).into();
        self.load_imm(imm)
    }

    fn visit_expr_if(&mut self, expr_if: &ExprIf) -> Reg {
        let ExprIf {
            if_token: _,
            cond,
            then_branch,
            else_branch: _,
        } = expr_if;
        // pub struct ExprIf {
        //     pub if_token: super::keyword::If,
        //     pub cond: Box<Expr>,
        //     pub then_branch: ExprBlock,
        //     pub else_branch: Option<(super::keyword::Else, Box<Expr>)>,
        // }
        let cond_reg = self.visit_expr(cond);
        let label = self.gen_label();
        let des = self.conditional(label.clone(), cond_reg);
        self.visit_expr_block(then_branch);
        self.def_label(label);
        des
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parse::parse;
    use pretty_assertions::assert_eq;

    fn setup(src: impl Into<String>) -> Vec<Instruction> {
        let tokens = lex(src.into().as_str()).unwrap();
        let ast = parse(tokens).unwrap();
        let mut gen = IrGenerator::default();
        gen.visit(&ast);
        gen.code
    }

    macro_rules! test_builder {
        (test_name: $name:ident, input: $input:expr, ir: $($t:expr, )* $(,)? ) => {
            #[test]
            fn $name() {
                let left = setup($input);
                let mut right: Vec<Instruction> = vec![];
                $(
                    right.push($t);
                )*
                assert_eq!(left, right);
            }
        };
    }

    test_builder! {
        test_name: test_binary_mul,
        input: "fn main() { 1+2*3; }",
        ir: DefFunc{
            name: "main".into(),
            ret: Type::I64,
            params: vec![],
            body: vec![
                Enter.into(),
                LoadImm{des: Reg(0), imm: Imm(1) }.into(),
                LoadImm{des: Reg(1), imm: Imm(2) }.into(),
                LoadImm{des: Reg(2), imm: Imm(3) }.into(),
                Mul {
                    des: Reg(3),
                    lhs: Reg(1),
                    rhs: Reg(2),
                }.into(),
                Add {
                    des: Reg(4),
                    lhs: Reg(0),
                    rhs: Reg(3),
                }.into(),
                DefLabel(".exit".into()).into(),
                Leave.into(),
            ],

        }.into(),
    }

    test_builder! {
        test_name: test_ir_gen,
        input: "fn main() { 1 + 2; }",
        ir: DefFunc{
            name: "main".into(),
            ret: Type::I64,
            params: vec![],
            body: vec![
                Enter.into(),
                LoadImm{des: Reg(0), imm: Imm(1) }.into(),
                LoadImm{des: Reg(1), imm: Imm(2) }.into(),
                Add {
                    des: Reg(2),
                    lhs: Reg(0),
                    rhs: Reg(1),
                }.into(),
                DefLabel(".exit".into()).into(),
                Leave.into(),
            ],

        }.into(),
    }

    test_builder! {
        test_name: test_ir_gen_calling,
        input: "fn add(x: u64, y: u64) -> u64 { x + y; } fn main() { add(1, 2); }",
        ir: DefFunc{
            name: "add".into(),
            ret: Type::I64,
            params: vec![
                (Reg(0), Type::I64),
                (Reg(1), Type::I64),
            ],
            body: vec![
                Enter.into(),
                Add {
                    des: Reg(2),
                    lhs: Reg(0),
                    rhs: Reg(1),
                }.into(),
                DefLabel(".exit".into()).into(),
                Leave.into(),
            ],

        }.into(),
        DefFunc{
            name: "main".into(),
            ret: Type::I64,
            params: vec![],
            body: vec![
                Enter.into(),
                LoadImm { des: Reg(0), imm: Imm(1) }.into(),
                LoadImm { des: Reg(1), imm: Imm(2) }.into(),
                Call {
                    caller: Label("add".into()),
                    args: vec![Reg(0), Reg(1)],
                    ret: Reg(1000),
                }.into(),
                DefLabel(".exit".into()).into(),
                Leave.into(),
            ],

        }.into(),
    }
}

// NOTE: make sure the args from call will be loaded in function def
