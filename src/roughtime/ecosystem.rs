//! https://roughtime.googlesource.com/roughtime/+/HEAD/ECOSYSTEM.md#curating-server-lists
//! So, instead, Roughtime is only available for products that can be updated. The server lists have
//! an explicit expiry time in them and we will actively seek to break clients that try to use old
//! information in order to maintain ecosystem health. At the moment changing the hostname or port
//! of a server is the easiest way to enforce this but we expect to add a per-server id in the
//! future that clients would need to send in order to prove to the server that they have a current
//! server list.
//! https://github.com/cloudflare/roughtime/blob/master/ecosystem.json

pub mod server;
pub mod env;
pub mod json;
use crate::error;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Ecosystem {
    servers: server::Servers,
}

impl AsRef<server::Servers> for Ecosystem {
    fn as_ref(&self) -> &server::Servers {
        &self.servers
    }
}

impl std::convert::TryFrom<env::EcosystemJsonFilePath> for Ecosystem {
    type Error = error::ProvenanceError;
    fn try_from(ecosystem_json_file_path: env::EcosystemJsonFilePath) -> Result<Self, Self::Error> {
        Ok(Ecosystem::try_from(json::Ecosystem::try_from(ecosystem_json_file_path)?)?)
    }
}

impl std::convert::TryFrom<json::Ecosystem> for Ecosystem {
    type Error = error::ProvenanceError;
    fn try_from(ecosystem: json::Ecosystem) -> Result<Self, Self::Error> {
        Ok(
            serde_json::from_str(&String::from(ecosystem))?
        )
    }
}

#[cfg(test)]
mod test {

    use std::convert::TryFrom;

    #[test]
    fn server_load() {
        let ecosystem = super::Ecosystem::try_from(super::env::EcosystemJsonFilePath);

        println!("{:?}", ecosystem);
    }

}
