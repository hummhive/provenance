pub mod address;
pub mod public_key;

/// arbitrary name to identify a server for humans
/// not needed for anything technical
#[derive(serde::Deserialize, Debug)]
struct Name(String);

/// collection of everything that defines a time server
#[derive(serde::Deserialize, Debug)]
struct Server {
    name: Name,
    #[serde(alias="publicKeyType")]
    public_key_type: public_key::KeyType,
    #[serde(alias="publicKey")]
    public_key: public_key::Key,
    addresses: address::Addresses,
}

impl AsRef<public_key::Key> for Server {
    fn as_ref(&self) -> &public_key::Key {
        &self.public_key
    }
}

/// list of all servers from an ecosystem.json
#[derive(serde::Deserialize, Debug)]
pub(crate) struct Servers(Vec<Server>);

impl AsRef<Vec<Server>> for Servers {
    fn as_ref(&self) -> &Vec<Server> {
        &self.0
    }
}
