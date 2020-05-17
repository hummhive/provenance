/// the server url to fetch times from
#[derive(serde::Deserialize, Debug)]
struct Url(String);

/// the only protocol currently used in ecosystem.json is udp
#[derive(serde::Deserialize, Debug)]
enum Protocol {
    #[serde(alias="udp")]
    Udp
}

/// simple collection of protocol and url to fetch times from
#[derive(serde::Deserialize, Debug)]
struct Address {
    protocol: Protocol,
    address: Url,
}

/// list of all addresses for a given time server
/// currently every time server in ecosystem.json has only one address
#[derive(serde::Deserialize, Debug)]
pub(crate) struct Addresses(Vec<Address>);
