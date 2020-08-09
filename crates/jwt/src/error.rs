#[derive(thiserror::Error, Debug)]
pub enum JwtError {
    #[error(transparent)]
    Idp(#[from] humm_provenance_idp::error::IdpError),

    #[error(transparent)]
    Crypto(#[from] humm_crypto::error::CryptoError),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error(transparent)]
    JwtParse(#[from] jwt_compact::ParseError),

    #[error(transparent)]
    Creation(#[from] jwt_compact::CreationError),

    #[error(transparent)]
    Validation(#[from] jwt_compact::ValidationError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
