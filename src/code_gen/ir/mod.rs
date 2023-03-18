mod basic_block;
mod block_type;
mod generator;
mod input;
mod label;
mod op;
mod reg;
mod value;

use crate::parse::{Binary, Expr, Item, Lit, LitBool, LitInt, Statement};
pub use basic_block::BasicBlock;
pub use block_type::{Assignment, BlockType};
pub use generator::IrGenerator;
pub use input::Input;
pub use label::Label;
pub use op::Op;
pub use reg::Reg;
pub use value::Value;

pub fn code_gen(ast: &Item) -> Vec<BasicBlock> {
    let mut basicblocks = vec![];
    let blocks = IrGenerator::default().compile(&ast);
    basicblocks.push(BasicBlock { blocks });
    basicblocks
}

trait Visitor {
    fn visit_litbool(&mut self, lit_bool: &LitBool) -> Value;
    fn visit_litint(&mut self, lit_int: &LitInt) -> Value;
    fn visit_binary(&mut self, binary: &Binary, blocks: &mut Vec<BlockType>) -> Reg;
    fn visit_lit(&mut self, lit: &Lit) -> Input;
    fn visit_expr(&mut self, expr: &Expr, blocks: &mut Vec<BlockType>) {
        match expr {
            Expr::Lit(..) => unimplemented!(),
            Expr::Binary(ref binary) => {
                self.visit_binary(binary, blocks);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: Statement);

    fn visit(&mut self, item: Item) {
        match item {
            Item::Statement(stmt) => self.visit_stmt(stmt),
        }
    }
}
