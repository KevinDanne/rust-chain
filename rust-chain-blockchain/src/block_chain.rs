use uuid::Uuid;

use crate::block::Block;

/// Represents a chain of blocks
pub struct BlockChain<'a> {
    id: Uuid,
    blocks: Vec<&'a Block>,
}

impl<'a> BlockChain<'a> {
    /// Creates a new BlockChain instance
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            blocks: Vec::new(),
        }
    }

    /// Returns the id of the blockchain
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the blocks of the chain
    pub fn blocks(&self) -> &Vec<&Block> {
        &self.blocks
    }

    /// Adds the given block to the chain
    pub fn add_block(&mut self, block: &'a Block) {
        self.blocks.push(block);
    }

    /// Verifies if the block chain is valid
    pub fn verify(&self) -> bool {
        for block in self.blocks.iter() {
            if !block.verify() {
                return false;
            }
        }

        true
    }
}
