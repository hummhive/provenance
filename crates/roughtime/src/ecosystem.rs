//! https://roughtime.googlesource.com/roughtime/+/HEAD/ECOSYSTEM.md#curating-server-lists
//! So, instead, Roughtime is only available for products that can be updated. The server lists have
//! an explicit expiry time in them and we will actively seek to break clients that try to use old
//! information in order to maintain ecosystem health. At the moment changing the hostname or port
//! of a server is the easiest way to enforce this but we expect to add a per-server id in the
//! future that clients would need to send in order to prove to the server that they have a current
//! server list.
//! https://github.com/cloudflare/roughtime/blob/master/ecosystem.json

pub mod server;

use crate::ecosystem::server::public_key;
use crate::error;
use std::convert::TryFrom;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Ecosystem {
    servers: server::Servers,
}

impl AsRef<server::Servers> for Ecosystem {
    fn as_ref(&self) -> &server::Servers {
        &self.servers
    }
}

impl From<&Ecosystem> for public_key::Keys {
    fn from(ecosystem: &Ecosystem) -> Self {
        Self::from(
            ecosystem
                .as_ref()
                .as_ref()
                .iter()
                .map(|s| s.as_ref())
                .cloned()
                .collect::<Vec<public_key::Key>>(),
        )
    }
}

impl TryFrom<&Ecosystem> for public_key::KeysPortable {
    type Error = error::RoughtimeError;
    fn try_from(ecosystem: &Ecosystem) -> Result<Self, Self::Error> {
        Self::try_from(&public_key::Keys::from(ecosystem))
    }
}

impl From<&Ecosystem> for public_key::KeysHash {
    fn from(ecosystem: &Ecosystem) -> Self {
        (&public_key::Keys::from(ecosystem)).into()
    }
}
