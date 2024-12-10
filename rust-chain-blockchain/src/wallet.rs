use std::error::Error;
use std::fmt::{self, Display};

use rsa::pkcs1::EncodeRsaPublicKey;
use uuid::Uuid;
use rsa::{RsaPublicKey, RsaPrivateKey, pkcs8::{self, EncodePublicKey}};

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
    pub fn create_transaction(&self, receiver_hash: String, amount: f64) -> Transaction {
        let id = Uuid::new_v4();
        // TODO REMOVE UNWRAP
        let sender_hash = self.public_key.to_pkcs1_pem(pkcs8::LineEnding::CRLF).unwrap();
        // TODO SIGNATURE
        let signature = String::new();
        Transaction::new(id, sender_hash, receiver_hash, amount, signature)
    }
}