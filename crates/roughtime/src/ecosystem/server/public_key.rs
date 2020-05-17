/// the only key type currently used in ecosystem.json is ed25519
pub(crate) enum KeyType {
    Ed25519
}

/// public keys in an ecosystem.json are 32 bytes as base64
pub(crate) struct Key([u8; 32]);
