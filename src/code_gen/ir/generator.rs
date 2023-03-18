use super::{
    block_type::{Assignment, BlockType},
    input::Input,
    op::Op,
    reg::Reg,
    value::Value,
};
use crate::parse::{
    Item,
    Statement,
    Lit,
    LitInt,
    LitBool,
    Expr,
    Binary,
};
#[derive(Debug, Default)]
pub struct IrGenerator {
    reg_counter: usize,
    code: Vec<BlockType>,
}

impl IrGenerator {
    fn get_reg(&mut self) -> Reg {
        let r = self.reg_counter;
        self.reg_counter += 1;
        Reg(r)
    }

    fn visit_litbool(&mut self, lit_bool: &LitBool) -> Value {
        Value((lit_bool.value.parse::<bool>().unwrap() as usize).to_string())
    }

    fn visit_litint(&mut self, lit_int: &LitInt) -> Value {
        Value(lit_int.value.to_string())
    }

    fn visit_binary(&mut self, binary: &Binary) -> Reg {
        let mut unfold_expr = |expr| match expr {
            Expr::Lit(ref lit) => self.visit_lit(lit),
            Expr::Binary(ref binary) => Input::Reg(self.visit_binary(binary)),
        };
        let lhs = unfold_expr(Clone::clone(&binary.left));
        let rhs = unfold_expr(Clone::clone(&binary.right));
        let des = self.get_reg();
        self.code.push(BlockType::Assignment(Assignment {
            des: des.clone(),
            op: Op::from(binary.op.kind()),
            x: lhs,
            y: rhs,
        }));
        des
    }

    fn visit_lit(&mut self, lit: &Lit) -> Input {
        match lit {
            Lit::Int(int) => Input::Value(self.visit_litint(int)),
            Lit::Bool(boolean) => Input::Value(self.visit_litbool(boolean)),
        }
    }
    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Lit(..) => unimplemented!(),
            Expr::Binary(ref binary) => {
                self.visit_binary(binary);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Statement) {
        let Statement { stmt, .. } = stmt;
        self.visit_expr(stmt)
    }

    pub fn compile(mut self, item: &Item) -> Vec<BlockType> {
        match item {
            Item::Statement(ref stmt) => self.visit_stmt(stmt),
        }
        self.code
    }
}
