use uuid::Uuid;

use crate::transaction::Transaction;

/// Represents a block of transactions
pub struct Block {
    id: Uuid,
    previous_id: Option<Uuid>,
    transactions: Vec<Transaction>,
    nounce: u32,
    hash: String,
}

impl Block {
    /// Creates a new block instance
    pub fn new(previous_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            previous_id,
            transactions: Vec::new(),
            nounce: 0,
            hash: String::new(),
        }
    }

    /// Returns the id of the block
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the previous id of the block
    pub fn previous_id(&self) -> &Option<Uuid> {
        &self.previous_id
    }

    /// Returns the transactions of the block
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    /// Returns the nounce of the block
    pub fn nounce(&self) -> &u32 {
        &self.nounce
    }

    /// Returns the hash of the block
    pub fn hash(&self) -> &String {
        &self.hash
    }

    /// Verifies is the block is valid
    pub fn verify(&self) -> bool {
        false
    }

    /// Adds the given transaction to the block
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    /// Mines the block
    pub fn mine(&mut self) {
        self.nounce = 1;
    }
}
