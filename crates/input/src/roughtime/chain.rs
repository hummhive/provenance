use super::client;
use humm_provenance_roughtime as roughtime;
use humm_provenance_roughtime::ecosystem;
use std::convert::TryInto;
use std::net::ToSocketAddrs;

impl From<&ChainItemInput> for roughtime::chain::Data {
    fn from(input: &ChainItemInput) -> Self {
        input.data.clone()
    }
}

impl From<&ChainItemInput> for roughtime::crypto::Nonce {
    fn from(input: &ChainItemInput) -> Self {
        let data_hash: roughtime::crypto::DataHash = (&input.data).into();
        let mut bytes = vec![];
        bytes.extend(data_hash.as_ref());
        bytes.extend(input.blind.as_ref());
        let inner = humm_crypto::sha512::Sha512Hash::from(&bytes);
        Self::from(inner)
    }
}

pub struct ChainItemInput {
    pub blind: roughtime::chain::Blind,
    pub data: roughtime::chain::Data,
    pub server: ecosystem::server::Server,
}

impl From<&ChainItemInput> for roughtime::ecosystem::server::public_key::Key {
    fn from(input: &ChainItemInput) -> Self {
        *input.server.as_ref()
    }
}

impl From<&ChainItemInput> for roughtime::chain::Blind {
    fn from(input: &ChainItemInput) -> Self {
        input.blind
    }
}

impl std::convert::TryFrom<&ChainItemInput> for std::net::SocketAddr {
    type Error = roughtime::error::RoughtimeError;
    fn try_from(input: &ChainItemInput) -> Result<Self, Self::Error> {
        let url: ecosystem::server::address::Url = (&input.server).try_into()?;
        match url.as_ref().to_socket_addrs()?.next() {
            Some(v) => Ok(v),
            None => Err(
                humm_provenance_roughtime::error::RoughtimeError::NoSocketAddrs(
                    url.as_ref().to_string(),
                ),
            ),
        }
    }
}

impl std::convert::TryFrom<&ChainItemInput> for roughtime::chain::ChainItem {
    type Error = roughtime::error::RoughtimeError;
    fn try_from(input: &ChainItemInput) -> Result<Self, Self::Error> {
        let addr: std::net::SocketAddr = input.try_into()?;
        let mut socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("Couldn't open UDP socket");
        let request: client::Request = input.try_into()?;

        socket.send_to(request.as_ref(), addr)?;

        let response: roughenough::RtMessage = client::receive_response(&mut socket)?;

        Ok(roughtime::chain::ChainItem {
            blind: input.into(),
            data: input.into(),
            pub_key: input.into(),
            response: (&response).try_into()?,
        })
    }
}
