use humm_provenance_core::json;
use humm_provenance_core::env;
use super::error;

#[derive(Debug)]
struct Ecosystem(json::Json);

impl From<json::Json> for Ecosystem {
    fn from(json: json::Json) -> Self {
        Self(json)
    }
}

impl std::convert::TryFrom<super::env::EcosystemJsonFilePath> for Ecosystem {
    type Error = error::EcosystemError;
    fn try_from(ecosystem_json_file_path: super::env::EcosystemJsonFilePath) -> Result<Self, Self::Error> {
        Ok(
            json::Json::try_from(
                std::fs::read_to_string(
                    String::from(
                        env::Val::try_from(
                            ecosystem_json_file_path
                        )?
                    )
                )?
            )?.into()
        )
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;
    use crate::ecosystem::env::EcosystemJsonFilePath;

    #[test]
    fn ecosystem_load() {
        let ecosystem = super::Ecosystem::try_from(EcosystemJsonFilePath).unwrap();

        println!("{:?}", ecosystem);
    }
}
