use humm_provenance_crypto as crypto;

#[derive(serde::Serialize, Clone)]
pub struct Signature(crypto::ed25519::signature::Ed25519Signature);

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
