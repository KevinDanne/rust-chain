use uuid::Uuid;
use rsa::{RsaPublicKey, RsaPrivateKey};

/// Represents a user of the blockchain
pub struct User {
    id: Uuid,
    name: String,
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl User {
    /// Creates a new user instance
    pub fn new(name: impl Into<String>) -> rsa::Result<Self> {
        let mut rng = rand::thread_rng();
        let bits = 2048;

        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self {
            id: Uuid::new_v4(),
            name: name.into(),
            private_key,
            public_key
        })
    }

    /// Returns the id of the user
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the name of the user
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the public key of the user
    pub fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }
}