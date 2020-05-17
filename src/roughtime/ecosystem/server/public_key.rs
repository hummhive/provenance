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
pub(crate) struct Key(crypto::Ed25519Key);

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
