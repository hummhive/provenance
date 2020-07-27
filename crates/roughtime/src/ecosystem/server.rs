pub mod address;
pub mod public_key;
use crate::error;

/// arbitrary name to identify a server for humans
/// not needed for anything technical
#[derive(serde::Deserialize, Debug, Clone)]
struct Name(String);

/// collection of everything that defines a time server
#[derive(Clone, serde::Deserialize, Debug)]
pub struct Server {
    name: Name,
    #[serde(alias = "publicKeyType")]
    public_key_type: public_key::KeyType,
    #[serde(alias = "publicKey")]
    public_key: public_key::Key,
    addresses: address::Addresses,
}

impl std::convert::TryFrom<&Server> for address::Url {
    type Error = error::RoughtimeError;
    fn try_from(server: &Server) -> Result<Self, Self::Error> {
        Ok(server
            .addresses
            .as_ref()
            .into_iter()
            .next()
            .ok_or(error::RoughtimeError::ServerMissingAddress)?
            .address
            .clone())
    }
}

impl AsRef<public_key::Key> for Server {
    fn as_ref(&self) -> &public_key::Key {
        &self.public_key
    }
}

/// list of all servers from an ecosystem.json
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Servers(Vec<Server>);

impl AsRef<Vec<Server>> for Servers {
    fn as_ref(&self) -> &Vec<Server> {
        &self.0
    }
}
