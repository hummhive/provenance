use serde::ser;
use serde::de;
use serde::de::Error;
use std::convert::TryInto;
use crate::error;

#[derive(Clone, Copy)]
pub struct Sha512Hash([u8; ring::digest::SHA512_OUTPUT_LEN]);

impl ser::Serialize for Sha512Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

impl From<&Vec<u8>> for Sha512Hash {
    fn from(bytes: &Vec<u8>) -> Self {
        let mut hash = [0; ring::digest::SHA512_OUTPUT_LEN];
            hash.copy_from_slice(&
            ring::digest::digest(&ring::digest::SHA512, &bytes).as_ref().to_owned()
        );
        Self(hash)
    }
}

impl std::convert::AsRef<[u8]> for Sha512Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ed25519PubKey([u8; ed25519_dalek::PUBLIC_KEY_LENGTH]);

impl From<Ed25519PubKey> for Vec<u8> {
    fn from(pub_key: Ed25519PubKey) -> Self {
        pub_key.0.to_vec()
    }
}

impl From<&ed25519_dalek::Keypair> for Ed25519PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.public.to_bytes())
    }
}

impl From<[u8; ed25519_dalek::PUBLIC_KEY_LENGTH]> for Ed25519PubKey {
    fn from(bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH]) -> Self {
        Self(bytes)
    }
}

impl std::convert::AsRef<[u8]> for Ed25519PubKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl <'de> de::Deserialize<'de> for Ed25519PubKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
    D: de::Deserializer<'de>,
    {
        let s: &str = de::Deserialize::deserialize(deserializer)?;
        let bytes: &[u8] = &base64::decode(s).map_err(|_| D::Error::invalid_value(de::Unexpected::Str(s), &"a string that is not base64 bytes"))?[..];
        Ok(Ed25519PubKey(bytes.try_into().map_err(|_| D::Error::invalid_value(de::Unexpected::Str(s), &"incorrect length public key"))?))
    }
}

impl ser::Serialize for Ed25519PubKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

pub struct Ed25519SecretKey([u8; ed25519_dalek::SECRET_KEY_LENGTH]);

impl From<&ed25519_dalek::Keypair> for Ed25519SecretKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.secret.to_bytes())
    }
}

#[derive(Clone, Copy)]
pub struct Ed25519Keypair([u8; ed25519_dalek::KEYPAIR_LENGTH]);

impl std::convert::TryFrom<&Ed25519Keypair> for Ed25519PubKey {
    type Error = error::ProvenanceError;
    fn try_from(keypair: &Ed25519Keypair) -> Result<Self, Self::Error> {
        Ok(Self::from(&ed25519_dalek::Keypair::try_from(keypair)?))
    }
}

impl From<&ed25519_dalek::Keypair> for Ed25519Keypair {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.to_bytes())
    }
}

impl std::convert::TryFrom<&Ed25519Keypair> for ed25519_dalek::Keypair {
    type Error = error::ProvenanceError;
    fn try_from(keypair: &Ed25519Keypair) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(&keypair.0)?)
    }
}

pub struct Ed25519Signature([u8; ed25519_dalek::SIGNATURE_LENGTH]);
