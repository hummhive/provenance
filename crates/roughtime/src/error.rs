use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoughtimeError {
    // #[error(transparent)]
    // Roughenough(roughenough::Error),

    #[error(transparent)]
    Json(#[from] serde_json::error::Error),

    #[error("a roughtime server is missing an address")]
    ServerMissingAddress,
}
