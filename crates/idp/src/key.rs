use crate::error;
use ed25519_dalek;
use humm_crypto as crypto;
use std::convert::TryInto;

#[derive(serde::Serialize)]
#[serde(transparent)]
#[derive(Clone, Copy)]
pub struct PubKey(crypto::ed25519::public::Ed25519PubKey);

impl From<&ed25519_dalek::Keypair> for PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

pub struct SecretKey(crypto::ed25519::secret::Ed25519SecretKey);

impl From<&ed25519_dalek::Keypair> for SecretKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

#[derive(Clone, Copy)]
pub struct Keypair(crypto::ed25519::keypair::Ed25519Keypair);

impl std::convert::TryFrom<&Keypair> for PubKey {
    type Error = error::IdpError;
    fn try_from(keypair: &Keypair) -> Result<Self, Self::Error> {
        Ok(Self((&keypair.0).try_into()?))
    }
}

impl From<&ed25519_dalek::Keypair> for Keypair {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

impl std::convert::TryFrom<&Keypair> for ed25519_dalek::Keypair {
    type Error = error::IdpError;
    fn try_from(keypair: &Keypair) -> Result<Self, Self::Error> {
        Ok(Self::try_from(&keypair.0)?)
    }
}
