use crate::env;
use crate::error;

const HUMM_PROVENANCE_ROUGHTIME_ECOSYSTEM_PATH: &str = "HUMM_PROVENANCE_ROUGHTIME_ECOSYSTEM_PATH";

#[derive(Clone, Copy)]
pub struct EcosystemJsonFilePath;

impl From<EcosystemJsonFilePath> for env::Key {
    fn from(_: EcosystemJsonFilePath) -> env::Key {
        HUMM_PROVENANCE_ROUGHTIME_ECOSYSTEM_PATH.into()
    }
}

impl std::convert::TryFrom<EcosystemJsonFilePath> for env::Val {
    type Error = error::ProvenanceError;
    fn try_from(ecosystem_json_file: EcosystemJsonFilePath) -> Result<Self, Self::Error> {
        let key: env::Key = ecosystem_json_file.into();
        Ok(std::env::var(String::from(key))?.into())
    }
}

impl env::Var for EcosystemJsonFilePath {}
