#[derive(thiserror::Error, Debug)]
pub enum JwtError {
    #[error(transparent)]
    Crypto(#[from] humm_provenance_crypto::error::CryptoError),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error(transparent)]
    JwtParse(#[from] jwt_compact::ParseError),
}
