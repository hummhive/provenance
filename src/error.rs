use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProvenanceError {

    #[error("json serialization problem: {0}")]
    Json(#[from] serde_json::error::Error),

    #[error("could not get env value from environment: {0}")]
    Env(#[from] std::env::VarError),

    #[error("system io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("could not create jwt: {0}")]
    Jwt(#[from] jwt_compact::CreationError),

    #[error("could not sign")]
    Signature
}

impl From<ed25519_dalek::SignatureError> for ProvenanceError {
    fn from(_: ed25519_dalek::SignatureError) -> Self {
        Self::Signature
    }
}
