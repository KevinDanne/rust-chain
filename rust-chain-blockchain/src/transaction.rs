use uuid::Uuid;

/// Represents a transaction
pub struct Transaction {
    id: Uuid,
    sender_hash: String,
    receiver_hash: String,
    amount: f64,
    signature: String
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(
        id: Uuid,
        sender_hash: impl Into<String>,
        receiver_hash: impl Into<String>,
        amount: f64,
        signature: String
    ) -> Self {
        Self {
            id,
            sender_hash: sender_hash.into(),
            receiver_hash: receiver_hash.into(),
            amount,
            signature
        }
    }

    /// Returns the id of the transaction
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the sender hash of the transaction
    pub fn sender_hash(&self) -> &String {
        &self.sender_hash
    }

    /// Returns the receiver hash of the transaction
    pub fn receiver_hash(&self) -> &String {
        &self.receiver_hash
    }

    /// Returns the amount of the transaction
    pub fn amount(&self) -> &f64 {
        &self.amount
    }

    /// Returns the signature of the transaction
    pub fn signature(&self) -> &String {
        &self.signature
    }
}