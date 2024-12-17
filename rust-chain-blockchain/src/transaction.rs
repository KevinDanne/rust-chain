use std::hash::{DefaultHasher, Hash, Hasher};

use rsa::{pkcs1, Pkcs1v15Sign};
use thiserror::Error;
use uuid::Uuid;

use crate::wallet::Wallet;

/// Represents an error enum for the creation and verification of a transaction
#[derive(Error, Debug)]
pub enum TransactionCreationError {
    #[error("Receiver and sender signature MUST be different")]
    SameReceiverAndSender,

    #[error(transparent)]
    RsaError(#[from] rsa::Error),

    #[error(transparent)]
    PKCSError(#[from] pkcs1::Error),
}

/// Represents a transaction
pub struct Transaction {
    id: Uuid,
    sender_hash: u64,
    receiver_hash: u64,
    amount: f64,
    signature: Vec<u8>,
}

impl Transaction {
    /// Tries to creates and verify a new transaction
    pub fn new(
        sender: &Wallet,
        receiver: &Wallet,
        amount: f64,
    ) -> Result<Self, TransactionCreationError> {
        let id = Uuid::new_v4();

        if receiver.public_key() == sender.public_key() {
            return Err(TransactionCreationError::SameReceiverAndSender);
        }

        let mut hasher = DefaultHasher::new();
        sender.private_key().hash(&mut hasher);
        let sender_hash = hasher.finish();

        hasher = DefaultHasher::new();
        receiver.private_key().hash(&mut hasher);
        let receiver_hash = hasher.finish();

        let signature_data = format!("{}{}{}", sender_hash, receiver_hash, amount);
        let signature = sender
            .private_key()
            .sign(Pkcs1v15Sign::new_unprefixed(), signature_data.as_bytes())?;

        Ok(Self {
            id,
            sender_hash,
            receiver_hash,
            amount,
            signature,
        })
    }

    /// Returns the id of the transaction
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the sender hash of the transaction
    pub fn sender_hash(&self) -> &u64 {
        &self.sender_hash
    }

    /// Returns the receiver hash of the transaction
    pub fn receiver_hash(&self) -> &u64 {
        &self.receiver_hash
    }

    /// Returns the amount of the transaction
    pub fn amount(&self) -> &f64 {
        &self.amount
    }

    /// Returns the signature of the transaction
    pub fn signature(&self) -> &Vec<u8> {
        &self.signature
    }
}
