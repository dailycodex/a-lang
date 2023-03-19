mod basic_block;
mod block_type;
mod generator;
mod input;
mod label;
mod op;
mod reg;
mod value;
mod var;

use crate::parse::Item;
pub use basic_block::BasicBlock;
pub use block_type::{IrBlock, Conditional, Assignment, BlockType};
pub use generator::IrGenerator;
pub use input::Input;
pub use label::Label;
pub use op::Op;
pub use reg::Reg;
pub use value::Value;
pub use var::Var;

pub fn code_gen(ast: &Item) -> Vec<BasicBlock> {
    IrGenerator::default().compile(&ast)
}
