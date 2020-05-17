pub mod address;
pub mod public_key;

/// arbitrary name to identify a server for humans
/// not needed for anything technical
struct Name(String);

/// collection of everything that defines a time server
struct Server {
    name: Name,
    public_key_type: public_key::KeyType,
    public_key: public_key::Key,
    addresses: address::Addresses,
}

/// list of all servers from an ecosystem.json
struct Servers(Vec<Server>);
