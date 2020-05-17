use thiserror::Error;
use humm_provenance_core::error;

#[derive(Error, Debug)]
pub enum EcosystemError {

    #[error("system io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("provenance error: {0}")]
    Provenance(#[from] error::CoreError)

}
