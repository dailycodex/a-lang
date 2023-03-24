mod instruction;
use std::collections::HashMap;

pub use instruction::*;

use crate::lexer::*;
use crate::parse::*;

pub fn code_gen(ast: Vec<Item>) -> Result<Vec<Instruction>, Vec<String>> {
    let mut gen = IrGenerator::default();
    gen.visit(&ast);
    Ok(gen.code)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

impl From<&Ident> for Label {
    fn from(value: &Ident) -> Self {
        Label(value.value())
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Reg(pub usize);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Var(pub String);

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Mem;
//
// impl From<Mem> for Either<Imm, Mem> {
//     fn from(value: Mem) -> Self {
//         Self::Right(value)
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imm(pub u64);
impl From<u64> for Imm {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

// impl From<Imm> for Either<Imm, Mem> {
//     fn from(value: Imm) -> Self {
//         Self::Left(value)
//     }
// }

trait Ir {
    fn load_imm(&mut self, imm: Imm) -> Reg;
    fn load_var(&mut self, var: Var) -> Reg;
    fn binary(&mut self, op: &Op, lhs: Reg, rhs: Reg) -> Reg;
    fn call(&mut self, label: Label, args: Vec<Reg>, ret: Reg) -> Reg;
}

trait AstVisitor: Ir {
    fn visit_expr_var(&mut self, expr_var: &ExprVar) -> Reg;
    fn visit_expr_call(&mut self, expr_call: &ExprCall) -> Reg;
    fn visit_expr_binary(&mut self, bin: &ExprBinary) -> Reg;
    fn visit_item_fn(&mut self, item_fn: &ItemFn);
    fn visit_lit_int(&mut self, lit_int: &LitInt) -> Reg;
    fn visit_lit_bool(&mut self, lit_bool: &LitBool) -> Reg;

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
            // Block(ExprBlock) => unimplemented!(),
            Expr::Binary(ref ebinary) => self.visit_expr_binary(ebinary),
            Expr::Call(ref ecall) => self.visit_expr_call(ecall),
            Expr::Var(evar) => self.visit_expr_var(evar),
        }
    }

    fn visit_stmt(&mut self, stmt: &Statement) {
        let Statement { stmt, .. } = stmt;
        self.visit_expr(stmt);
    }

    fn visit_block(&mut self, block: &Block) {
        for stmt in block.stmts.iter() {
            self.visit_stmt(stmt);
        }
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
    reg_counter: usize,
    discriper: HashMap<Var, Reg>,
}

impl IrGenerator {
    fn store_var(&mut self, var: Var, reg: Reg) {
        self.discriper.insert(var, reg);
    }

    fn push(&mut self, ir: impl Into<Instruction>) {
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
}

impl Ir for IrGenerator {
    fn load_imm(&mut self, imm: Imm) -> Reg {
        let des = self.get_reg();
        let load = LoadImm { des, imm };
        self.push(load);
        des
    }
    fn load_var(&mut self, var: Var) -> Reg {
        let des = self.get_reg();
        let load = LoadVar { des, var };
        self.push(load);
        des
    }
    fn binary(&mut self, op: &Op, lhs: Reg, rhs: Reg) -> Reg {
        let des = self.get_reg();
        let instruction: Instruction = match op {
            Op::Add(_) => Add { des, lhs, rhs }.into(),

            Op::Sub(_) => Sub { des, lhs, rhs }.into(),
            Op::Mul(_) => Mul { des, lhs, rhs }.into(),
            Op::Div(_) => Div { des, lhs, rhs }.into(),
            _ => unimplemented!(),
        };
        self.push(instruction);
        des
    }
    fn call(&mut self, caller: Label, args: Vec<Reg>, ret: Reg) -> Reg {
        let instruction: Instruction = Call { caller, args, ret }.into();
        self.push(instruction);
        ret
    }
}

impl AstVisitor for IrGenerator {
    fn visit_expr_var(&mut self, expr_var: &ExprVar) -> Reg {
        let name = expr_var.name.value();
        self.load_var(Var(name))
    }

    fn visit_lit_int(&mut self, lit_int: &LitInt) -> Reg {
        let imm: Imm = lit_int.parse::<u64>().unwrap().into();
        self.load_imm(imm)
    }

    fn visit_lit_bool(&mut self, lit_bool: &LitBool) -> Reg {
        let imm: Imm = lit_bool.parse::<u64>().unwrap().into();
        self.load_imm(imm)
    }

    fn visit_expr_binary(&mut self, bin: &ExprBinary) -> Reg {
        let ExprBinary {
            left, right, op, ..
        } = bin;
        let lhs = self.visit_expr(left);
        let rhs = self.visit_expr(right);
        let des = self.binary(op, lhs, rhs);
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

    fn visit_item_fn(&mut self, item_fn: &ItemFn) {
        let ItemFn {
            name,
            params,
            block,
            ret_type,
            ..
        } = item_fn;
        let vars = params
            .iter()
            .map(|p| Var(p.name.value()))
            .collect::<Vec<Var>>();
        for var in vars {
            let reg = self.get_reg();
            self.store_var(var, reg);
        }
        self.reset_regester_count();
        self.push(DefLabel { label: name.into() });
        self.push(Enter);
        self.visit_block(&block);
        self.push(Leave);
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
        ($name:ident, $input:expr $(, $t:expr)* $(,)? ) => {
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
        test_binary_mul,
        "fn main() { 1+2*3; }",
        DefLabel {label: Label("main".into()).into()}.into(),
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
        Leave.into()
    }

    test_builder! {
        test_ir_gen,
        "fn main() { 1 + 2; }",
        DefLabel{label:Label("main".into())}.into(),
        Enter.into(),
        LoadImm{des: Reg(0), imm: Imm(1) }.into(),
        LoadImm{des: Reg(1), imm: Imm(2) }.into(),
        Add {
            des: Reg(2),
            lhs: Reg(0),
            rhs: Reg(1),
        }.into(),
        Leave.into(),
    }

    test_builder! {
        test_ir_gen_calling,
        "fn add(x: u64, y: u64) -> u64 { x + y; } fn main() { add(1, 2); }",
        DefLabel{label:Label("add".into())}.into(),
        Enter.into(),
        LoadVar{des: Reg(0), var: Var("x".into())}.into(),
        LoadVar{des: Reg(1), var: Var("y".into())}.into(),
        Add {
            des: Reg(2),
            lhs: Reg(0),
            rhs: Reg(1),
        }.into(),
        Leave.into(),

        DefLabel{label:Label("main".into())}.into(),
        Enter.into(),
        LoadImm{des: Reg(0), imm: Imm(1) }.into(),
        LoadImm{des: Reg(1), imm: Imm(2) }.into(),
        Call {
            caller: Label("add".into()),
            args: vec![Reg(0), Reg(1)],
            ret: Reg(1000),
        }.into(),
        Leave.into(),
    }
}

// NOTE: make sure the args from call will be loaded in function def