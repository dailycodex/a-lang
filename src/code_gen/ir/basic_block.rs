use super::block_type::BlockType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicBlock {
    pub blocks: Vec<BlockType>,
    pub comment: Option<String>,
}

impl BasicBlock {
    pub fn new(blocks: Vec<BlockType>) -> Self {
        Self {
            blocks,
            comment: None,
        }
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

impl From<Vec<BlockType>> for BasicBlock {
    fn from(value: Vec<BlockType>) -> Self {
        Self::new(value)
    }
}

impl From<(Vec<BlockType>, String)> for BasicBlock {
    fn from((blocks, comment): (Vec<BlockType>, String)) -> Self {
        Self::new(blocks).with_comment(comment)
    }
}

impl From<(String, Vec<BlockType>)> for BasicBlock {
    fn from((comment, blocks): (String, Vec<BlockType>)) -> Self {
        Self::new(blocks).with_comment(comment)
    }
}
