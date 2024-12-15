use std::{
    error::Error,
    fmt::{self, Display},
};

use rsa::{
    pkcs1::{self, EncodeRsaPublicKey},
    pkcs8::LineEnding,
};
use uuid::Uuid;

use crate::wallet::Wallet;

/// Represents an error enum for the creation and verification of a transaction
#[derive(Debug)]
pub enum TransactionCreationError {
    InvalidSenderSignature,
    PKCSError(pkcs1::Error),
}

impl Display for TransactionCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionCreationError::InvalidSenderSignature => {
                write!(f, "Receiver signature is invalid")?
            }
            Self::PKCSError(err) => write!(f, "{}", err)?,
        };
        Ok(())
    }
}

impl Error for TransactionCreationError {}

impl From<pkcs1::Error> for TransactionCreationError {
    fn from(value: pkcs1::Error) -> Self {
        Self::PKCSError(value)
    }
}
/// Represents a transaction
pub struct Transaction {
    id: Uuid,
    sender_hash: String,
    receiver_hash: String,
    amount: f64,
    signature: String,
}

impl Transaction {
    /// Tries to creates and verify a new transaction
    pub fn new(
        sender: &Wallet,
        receiver: &Wallet,
        amount: f64,
    ) -> Result<Self, TransactionCreationError> {
        let id = Uuid::new_v4();

        let receiver_hash = receiver.public_key().to_pkcs1_pem(LineEnding::CRLF)?;
        let sender_hash = sender.public_key().to_pkcs1_pem(LineEnding::CRLF)?;

        // TODO SIGNATURE
        let signature = String::new();

        // TODO add verify
        Ok(Self {
            id,
            sender_hash: sender_hash.into(),
            receiver_hash: receiver_hash.into(),
            amount,
            signature,
        })
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
