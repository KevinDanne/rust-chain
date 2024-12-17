use std::{
    error::Error,
    fmt::{self, Display},
    hash::{DefaultHasher, Hash, Hasher},
};

use rsa::{pkcs1, Pkcs1v15Sign};
use uuid::Uuid;

use crate::wallet::Wallet;

/// Represents an error enum for the creation and verification of a transaction
#[derive(Debug)]
pub enum TransactionCreationError {
    SameReceiverAndSender,
    RsaError(rsa::Error),
    PKCSError(pkcs1::Error),
}

impl Display for TransactionCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SameReceiverAndSender => {
                write!(f, "Receiver and sender signature MUST be different")?
            }
            Self::RsaError(err) => write!(f, "{}", err)?,
            Self::PKCSError(err) => write!(f, "{}", err)?,
        };
        Ok(())
    }
}

impl Error for TransactionCreationError {}

impl From<rsa::Error> for TransactionCreationError {
    fn from(value: rsa::Error) -> Self {
        Self::RsaError(value)
    }
}

impl From<pkcs1::Error> for TransactionCreationError {
    fn from(value: pkcs1::Error) -> Self {
        Self::PKCSError(value)
    }
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
