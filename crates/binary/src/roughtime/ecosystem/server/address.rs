/// the server url to fetch times from
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Url(String);

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// the only protocol currently used in ecosystem.json is udp
#[derive(serde::Deserialize, Debug, Clone, Copy)]
enum Protocol {
    #[serde(alias = "udp")]
    Udp,
}

/// simple collection of protocol and url to fetch times from
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Address {
    protocol: Protocol,
    pub address: Url,
}

/// list of all addresses for a given time server
/// currently every time server in ecosystem.json has only one address
#[derive(serde::Deserialize, Debug, Clone)]
pub(crate) struct Addresses(Vec<Address>);

impl AsRef<[Address]> for Addresses {
    fn as_ref(&self) -> &[Address] {
        &self.0
    }
}
