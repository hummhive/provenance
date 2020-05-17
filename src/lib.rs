pub mod env;
pub mod json;
pub mod error;
pub mod crypto;
pub mod content;
pub mod version;
pub mod roughtime;
pub mod jwt;
pub mod device;

#[derive(serde::Serialize)]
struct Provenance {
    pub version: version::Version,
    pub content_hash: content::Hash,
    pub roughtime: roughtime::Roughtime,
    pub jwt: jwt::Jwt,
    pub device_signature: device::Signature,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn make_provenance() {
        let ecosystem = roughtime::ecosystem::Ecosystem::try_from(roughtime::ecosystem::env::EcosystemJsonFilePath).unwrap();
        let content = content::Content::from(b"foo".to_vec());

        let provenance = Provenance {
            version: version::Version::One,
            content_hash: (&content).into(),
            roughtime: roughtime::Roughtime {
                public_keys: (&ecosystem).into(),
            },
            jwt: jwt::Jwt,
            device_signature: device::Signature,
        };

        let output_path = std::env::var("HUMM_PROVENANCE_OUTPUT_PATH").unwrap();

        let json_output = serde_json::to_string_pretty(&provenance).unwrap();
        println!("{}", json_output);
        std::fs::write(output_path, json_output).unwrap();
    }

}
