#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("could not sign")]
    Signature,

    #[error(transparent)]
    Json(#[from] serde_json::error::Error),
}

impl From<ed25519_dalek::SignatureError> for CryptoError {
    fn from(_: ed25519_dalek::SignatureError) -> Self {
        Self::Signature
    }
}
