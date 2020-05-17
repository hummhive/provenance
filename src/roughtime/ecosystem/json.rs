use crate::json;
use crate::env;
use crate::error;

#[derive(Debug)]
pub(crate) struct Ecosystem(json::Json);

impl From<json::Json> for Ecosystem {
    fn from(json: json::Json) -> Self {
        Self(json)
    }
}

impl From<Ecosystem> for String {
    fn from(ecosystem: Ecosystem) -> String {
        String::from(ecosystem.0)
    }
}

impl std::convert::TryFrom<super::env::EcosystemJsonFilePath> for Ecosystem {
    type Error = error::ProvenanceError;
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
    use crate::roughtime::ecosystem::env::EcosystemJsonFilePath;

    #[test]
    fn ecosystem_load() {
        let ecosystem = super::Ecosystem::try_from(EcosystemJsonFilePath).unwrap();

        println!("{:?}", ecosystem);
    }
}
