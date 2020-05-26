use crate::crypto;
use crate::error;
use std::convert::TryFrom;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct PubKey(crypto::Ed25519PubKey);

impl From<&ed25519_dalek::Keypair> for PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

pub struct PubKeyPortable(String);

impl TryFrom<&ed25519_dalek::Keypair> for PubKeyPortable {
    type Error = error::ProvenanceError;
    fn try_from(keypair: &ed25519_dalek::Keypair) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::to_string(&PubKey::from(keypair))?))
    }
}

impl TryFrom<&PubKeyPortable> for PubKey {
    type Error = error::ProvenanceError;
    fn try_from(pub_key_portable: &PubKeyPortable) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&pub_key_portable.0)?)
    }
}

pub struct Keypair(crypto::Ed25519Keypair);

impl From<&ed25519_dalek::Keypair> for Keypair {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

impl From<&Keypair> for crypto::Ed25519Keypair {
    fn from(keypair: &Keypair) -> Self {
        keypair.0
    }
}
