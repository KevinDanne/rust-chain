use std::error::Error;
use std::fmt::{self, Display};

use uuid::Uuid;
use rsa::{RsaPublicKey, RsaPrivateKey, pkcs1::{self, EncodeRsaPublicKey}, pkcs8::LineEnding};

use crate::transaction::Transaction;

const PRIVATE_KEY_BIT_SIZE: usize = 2048;

/// Represents an error enum for the creation of a new wallet
#[derive(Debug)]
pub enum WalletCreationError {
    RsaError(rsa::Error)
}

impl Display for WalletCreationError {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletCreationError::RsaError(err) => write!(f, "{}", err)?
        };
        Ok(()) 
   } 
}

impl Error for WalletCreationError {}

impl From<rsa::Error> for WalletCreationError {
    fn from(value: rsa::Error) -> Self {
        Self::RsaError(value)
    }
}

/// Represents an error enum for the creation of a new transaction
#[derive(Debug)]
pub enum TransactionCreationError {
    PKCSError(pkcs1::Error)
}

impl Display for TransactionCreationError {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionCreationError::PKCSError(err) => write!(f, "{}", err)?
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

/// Represents a user wallet
pub struct Wallet {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl Wallet {
    /// Creates a new wallet instance
    pub fn new() -> Result<Self, WalletCreationError> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, PRIVATE_KEY_BIT_SIZE)?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self {
            private_key,
            public_key
        })
    }

    /// Returns the public key of the user
    pub fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

    /// Creates a new signed transaction
    pub fn create_transaction(&self, receiver_public_key: &RsaPublicKey, amount: f64) -> Result<Transaction, TransactionCreationError> {
        let id = Uuid::new_v4();

        let receiver_hash = receiver_public_key.to_pkcs1_pem(LineEnding::CRLF)?;
        let sender_hash = self.public_key.to_pkcs1_pem(LineEnding::CRLF)?;

        // TODO SIGNATURE
        let signature = String::new();
        
        Ok(Transaction::new(id, sender_hash, receiver_hash, amount, signature))
    }
}