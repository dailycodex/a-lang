use super::block_type::BlockType;

#[derive(Debug, PartialEq, Eq)]
pub struct BasicBlock {
    pub blocks: Vec<BlockType>,
}
