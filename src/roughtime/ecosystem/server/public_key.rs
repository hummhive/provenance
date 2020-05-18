use crate::roughtime::ecosystem;
use crate::crypto;

/// the only key type currently used in ecosystem.json is ed25519
#[derive(serde::Deserialize, Debug)]
pub(crate) enum KeyType {
    #[serde(alias="ed25519")]
    Ed25519
}

/// public keys in an ecosystem.json are 32 bytes as base64
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub(crate) struct Key(crypto::Ed25519PubKey);

impl From<Key> for Vec<u8> {
    fn from(key: Key) -> Self {
        key.0.into()
    }
}

#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct Keys(Vec<Key>);

impl From<&ecosystem::Ecosystem> for Keys {
    fn from(ecosystem: &ecosystem::Ecosystem) -> Self {
        Self(
            ecosystem.as_ref().as_ref().iter().map(|s| s.as_ref()).cloned().collect()
        )
    }
}

#[derive(serde::Serialize, Clone, Copy)]
#[serde(transparent)]
pub struct KeysHash(crypto::Sha512Hash);

impl From<&ecosystem::Ecosystem> for KeysHash {
    fn from(ecosystem: &ecosystem::Ecosystem) -> Self {
        let mut bytes: Vec<u8> = vec![];
        let keys = Keys::from(ecosystem);
        for key in keys.0 {
            let mut key_bytes: Vec<u8> = key.into();
            bytes.append(&mut key_bytes);
        }
        Self((&bytes).into())
    }
}
