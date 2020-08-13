#[derive(thiserror::Error, Debug)]
pub enum IdpError {
    #[error(transparent)]
    Crypto(#[from] humm_crypto::error::CryptoError),
    // #[error(transparent)]
    // Json(#[from] serde_json::error::Error),
}
