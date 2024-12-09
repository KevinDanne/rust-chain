use uuid::Uuid;

use crate::transaction::Transaction;

/// Represents a block of transactions
pub struct Block {
    id: Uuid,
    previous_id: Option<Uuid>,
    transactions: Vec<Transaction>,
    proof_of_work: u32,
}

impl Block {
    /// Creates a new block instance
    pub fn new(
        id: Uuid,
        previous_id: Option<Uuid>,
        transactions: impl Into<Vec<Transaction>>,
        proof_of_work: u32
    ) -> Self {
        Self {
            id,
            previous_id,
            transactions: transactions.into(),
            proof_of_work
        }
    }
}