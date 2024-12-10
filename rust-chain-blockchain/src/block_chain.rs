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

    /// Returns the blocks of the chain
    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    /// Adds the given block to the chain
    pub fn add_block(&mut self, block: Block) {
        // TODO verify block
        self.blocks.push(block);
    }
}