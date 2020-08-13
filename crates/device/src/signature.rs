use humm_crypto as crypto;

#[derive(serde::Serialize, Clone)]
pub struct Signature(crypto::ed25519::signature::Ed25519Signature);

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<crypto::ed25519::signature::Ed25519Signature> for Signature {
    fn from(ed25519_sig: crypto::ed25519::signature::Ed25519Signature) -> Self {
        Self(ed25519_sig)
    }
}
