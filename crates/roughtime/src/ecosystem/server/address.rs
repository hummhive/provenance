/// the server url to fetch times from
#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Url(String);

#[cfg(test)]
#[test]
fn url_serde() {
    let url = Url(String::from("foo"));
    let s = serde_json::to_string(&url).unwrap();

    assert_eq!(r#""foo""#, s,);

    let r: Url = serde_json::from_str(&s).unwrap();

    assert_eq!(r, url,);
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn url_as_ref_str() {
    let i = "foo";
    assert_eq!(Url(i.into()).as_ref(), i,);
}

/// the only protocol currently used in ecosystem.json is udp
#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
enum Protocol {
    #[serde(rename = "udp")]
    Udp,
}

#[cfg(test)]
#[test]
fn protocol_serde() {
    let protocol = Protocol::Udp;
    let s = serde_json::to_string(&protocol).unwrap();

    assert_eq!(r#""udp""#, s);

    let r: Protocol = serde_json::from_str(&s).unwrap();

    assert_eq!(r, protocol);
}

/// simple collection of protocol and url to fetch times from
#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Address {
    protocol: Protocol,
    pub address: Url,
}

#[cfg(test)]
impl Address {
    fn fixt_foo() -> Self {
        Self {
            protocol: Protocol::Udp,
            address: Url(String::from("foo")),
        }
    }

    fn fixt_bar() -> Self {
        Self {
            protocol: Protocol::Udp,
            address: Url(String::from("bar")),
        }
    }
}

#[cfg(test)]
#[test]
fn address_serde() {
    let address = Address::fixt_foo();

    let s = serde_json::to_string(&address).unwrap();

    assert_eq!(r#"{"protocol":"udp","address":"foo"}"#, s);

    let r: Address = serde_json::from_str(&s).unwrap();

    assert_eq!(r, address);
}

/// list of all addresses for a given time server
/// currently every time server in ecosystem.json has only one address
#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub(crate) struct Addresses(Vec<Address>);

#[cfg(test)]
#[test]
fn addresses_serde() {
    let addresses = Addresses(vec![Address::fixt_foo(), Address::fixt_bar()]);

    let s = serde_json::to_string(&addresses).unwrap();

    assert_eq!(
        r#"[{"protocol":"udp","address":"foo"},{"protocol":"udp","address":"bar"}]"#,
        s,
    );

    let r: Addresses = serde_json::from_str(&s).unwrap();

    assert_eq!(r, addresses);
}

impl AsRef<[Address]> for Addresses {
    fn as_ref(&self) -> &[Address] {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn addresses_as_ref_slice() {
    let inner = vec![Address::fixt_foo(), Address::fixt_bar()];
    let addresses = Addresses(inner.clone());
    assert_eq!(inner.as_slice(), addresses.as_ref(),);
}
