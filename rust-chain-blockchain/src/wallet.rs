use std::{
    error::Error,
    fmt::{self, Display},
};

use rsa::{RsaPrivateKey, RsaPublicKey};

use crate::transaction::{Transaction, TransactionCreationError};

const PRIVATE_KEY_BIT_SIZE: usize = 2048;

/// Represents an error enum for the creation of a new wallet
#[derive(Debug)]
pub enum WalletCreationError {
    RsaError(rsa::Error),
}

impl Display for WalletCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletCreationError::RsaError(err) => write!(f, "{}", err)?,
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
            public_key,
        })
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
