use humm_provenance_crypto as crypto;
use crate::error;
use ed25519_dalek;
use humm_provenance_crypto::ed25519::public::Ed25519PubKey;

#[derive(serde::Serialize)]
#[serde(transparent)]
#[derive(Clone, Copy)]
pub struct PubKey(crypto::Ed25519PubKey);

impl From<&ed25519_dalek::Keypair> for PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

pub struct SecretKey(crypto::Ed25519SecretKey);

impl From<&ed25519_dalek::Keypair> for SecretKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

#[derive(Clone, Copy)]
pub struct Keypair(crypto::Ed25519Keypair);

impl std::convert::TryFrom<&Keypair> for PubKey {
    type Error = error::ProvenanceError;
    fn try_from(keypair: &Keypair) -> Result<Self, Self::Error> {
        Ok(Self((&keypair.0).try_into()?))
    }
}

impl From<&crate::JwtInput> for Keypair {
    fn from(input: &crate::JwtInput) -> Self {
        input.idp_keypair
    }
}

impl From<&ed25519_dalek::Keypair> for Keypair {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.into())
    }
}

impl std::convert::TryFrom<&Keypair> for ed25519_dalek::Keypair {
    type Error = error::ProvenanceError;
    fn try_from(keypair: &Keypair) -> Result<Self, Self::Error> {
        Ok(Self::try_from(&keypair.0)?)
    }
}
