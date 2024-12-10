use uuid::Uuid;

use crate::transaction::Transaction;

/// Represents a block of transactions
pub struct Block {
    id: Uuid,
    previous_id: Option<Uuid>,
    transactions: Vec<Transaction>,
    proof_of_work_salt: Option<u32>,
    proof_of_work_hash: Option<String>,
}

impl Block {
    /// Creates a new block instance
    pub fn new(
        previous_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            previous_id,
            transactions: Vec::new(),
            proof_of_work_salt: None,
            proof_of_work_hash: None,
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

    /// Returns the proof_of_work_salt of the block
    pub fn proof_of_work_salt(&self) -> &Option<u32> {
        &self.proof_of_work_salt
    }

    /// Returns the proof_of_work_hash of the block
    pub fn proof_of_work_hash(&self) -> &Option<String> {
        &self.proof_of_work_hash
    }

    /// Adds the given transaction to the block
    pub fn add_transaction(&mut self, transaction: Transaction) {
        // TODO verify transaction
        self.transactions.push(transaction);
    }
}