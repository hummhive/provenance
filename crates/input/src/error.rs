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

    #[error("unspecified ring crypto error: {0}")]
    RingUnspecified(#[from] ring::error::Unspecified),

    #[error("failed to roughtime after max attempts")]
    RoughtimeMaxAttempts,

    #[error("failed to create socket address for server: {0}")]
    NoSocketAddrs(String),

    #[error("roughenough error: {0}")]
    Roughenough(String),
}

impl From<roughenough::Error> for ProvenanceError {
    fn from(error: roughenough::Error) -> Self {
        Self::Roughenough(format!("{:?}", error))
    }
}
