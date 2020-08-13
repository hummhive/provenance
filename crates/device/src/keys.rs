use humm_crypto as crypto;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct PubKey(crypto::ed25519::public::Ed25519PubKey);

impl From<&ed25519_dalek::Keypair> for PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

pub struct Keypair(crypto::ed25519::keypair::Ed25519Keypair);

impl From<&ed25519_dalek::Keypair> for Keypair {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

impl From<&Keypair> for crypto::ed25519::keypair::Ed25519Keypair {
    fn from(keypair: &Keypair) -> Self {
        keypair.0
    }
}
