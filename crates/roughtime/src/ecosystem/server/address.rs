/// the server url to fetch times from
struct Url(String);

/// the only protocol currently used in ecosystem.json is udp
enum Protocol {
    Udp
}

/// simple collection of protocol and url to fetch times from
struct Address {
    protocol: Protocol,
    address: Url,
}

/// list of all addresses for a given time server
/// currently every time server in ecosystem.json has only one address
pub(crate) struct Addresses(Vec<Address>);
