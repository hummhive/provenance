use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoughtimeError {
    #[error(transparent)]
    Json(#[from] serde_json::error::Error),

    #[error("a roughtime server is missing an address")]
    ServerMissingAddress,

    #[error("no socket at address {0}")]
    NoSocketAddrs(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[cfg(feature = "client")]
    #[error("roughenough error: {0}")]
    Roughenough(String),
}

#[cfg(feature = "client")]
impl From<roughenough::Error> for RoughtimeError {
    fn from(error: roughenough::Error) -> Self {
        Self::Roughenough(format!("{:?}", error))
    }
}
