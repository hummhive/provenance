use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonError {
    #[error(transparent)]
    Json(#[from] serde_json::error::Error),
}
