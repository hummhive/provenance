use crate::crypto;
use crate::error;
use crate::roughtime::ecosystem;
use std::convert::TryFrom;

/// the only key type currently used in ecosystem.json is ed25519
#[derive(serde::Deserialize, Debug, Clone)]
pub(crate) enum KeyType {
    #[serde(alias = "ed25519")]
    Ed25519,
}

/// public keys in an ecosystem.json are 32 bytes as base64
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub(crate) struct Key(crypto::Ed25519PubKey);

impl From<&Key> for [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] {
    fn from(key: &Key) -> Self {
        (&key.0).into()
    }
}

#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct Keys(Vec<Key>);

impl From<&ecosystem::Ecosystem> for Keys {
    fn from(ecosystem: &ecosystem::Ecosystem) -> Self {
        Self(
            ecosystem
                .as_ref()
                .as_ref()
                .iter()
                .map(|s| s.as_ref())
                .cloned()
                .collect(),
        )
    }
}

pub struct KeysPortable(String);

impl TryFrom<&ecosystem::Ecosystem> for KeysPortable {
    type Error = error::ProvenanceError;
    fn try_from(ecosystem: &ecosystem::Ecosystem) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::to_string(&Keys::from(ecosystem))?))
    }
}

impl TryFrom<&KeysPortable> for Keys {
    type Error = error::ProvenanceError;
    fn try_from(keys_portable: &KeysPortable) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::from_str(&keys_portable.0)?))
    }
}

#[derive(serde::Serialize, Clone, Copy)]
#[serde(transparent)]
pub struct KeysHash(crypto::Sha512Hash);

impl From<&Keys> for KeysHash {
    fn from(keys: &Keys) -> Self {
        let mut bytes: Vec<u8> = vec![];
        for key in &keys.0 {
            let key_bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = key.into();
            bytes.extend(key_bytes.iter());
        }
        Self((&bytes).into())
    }
}

impl From<&ecosystem::Ecosystem> for KeysHash {
    fn from(ecosystem: &ecosystem::Ecosystem) -> Self {
        (&Keys::from(ecosystem)).into()
    }
}
