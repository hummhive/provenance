use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error(transparent)]
    Json(#[from] serde_json::error::Error),
}
