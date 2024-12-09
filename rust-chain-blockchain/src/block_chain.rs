use crate::block::Block;

/// Represents a chain of blocks
pub struct BlockChain {
    blocks: Vec<Block>
}

impl BlockChain {
    /// Creates a new BlockChain instance
    pub fn new() -> Self {
        Self {
            blocks: Vec::new()
        }
    }

    /// Adds a new block to the chain
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}