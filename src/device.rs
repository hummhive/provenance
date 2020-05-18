use crate::crypto;

#[derive(serde::Serialize)]
pub struct Signature;

#[derive(serde::Serialize, Clone, Copy)]
#[serde(transparent)]
pub struct PubKey(crypto::Ed25519PubKey);

impl From<&ed25519_dalek::Keypair> for PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}
