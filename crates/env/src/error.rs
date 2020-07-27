use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("could not get env value from environment: {0}")]
    Env(#[from] std::env::VarError),
}
