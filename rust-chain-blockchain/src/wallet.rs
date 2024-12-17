use rsa::{RsaPrivateKey, RsaPublicKey};
use thiserror::Error;

use crate::transaction::{Transaction, TransactionCreationError};

const PRIVATE_KEY_BIT_SIZE: usize = 2048;

/// Represents an error enum for the creation of a new wallet
#[derive(Error, Debug)]
pub enum WalletCreationError {
    #[error(transparent)]
    RsaError(#[from] rsa::Error),
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
            public_key,
        })
    }

    /// Returns the private key of the user
    pub fn private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }

    /// Returns the public key of the user
    pub fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

    /// Creates a new transaction.
    /// This is a wrapper around Transaction::new
    pub fn create_transaction(
        &self,
        receiver: &Wallet,
        amount: f64,
    ) -> Result<Transaction, TransactionCreationError> {
        Transaction::new(self, receiver, amount)
    }
}
