use crate::error;
use humm_provenance_crypto as crypto;
use std::convert::TryFrom;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct PubKey(crypto::ed25519::public::Ed25519PubKey);

impl From<&ed25519_dalek::Keypair> for PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

pub struct PubKeyPortable(String);

impl TryFrom<&ed25519_dalek::Keypair> for PubKeyPortable {
    type Error = error::DeviceError;
    fn try_from(keypair: &ed25519_dalek::Keypair) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::to_string(&PubKey::from(keypair))?))
    }
}

impl TryFrom<&PubKeyPortable> for PubKey {
    type Error = error::DeviceError;
    fn try_from(pub_key_portable: &PubKeyPortable) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&pub_key_portable.0)?)
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
