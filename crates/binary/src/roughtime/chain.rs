use crate::crypto;
use crate::error;
use crate::roughtime::client;
use crate::roughtime::ecosystem;
use crate::roughtime::ecosystem::server;
use ring::rand::SecureRandom;
use std::convert::TryInto;
use std::net::ToSocketAddrs;

// the blind to make the hash unpredictable for the server
const BLIND_LEN: usize = ring::digest::SHA512_OUTPUT_LEN;
#[derive(Clone, Copy)]
pub struct Blind([u8; BLIND_LEN]);

impl serde::Serialize for Blind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

impl Default for Blind {
    fn default() -> Self {
        let rng = ring::rand::SystemRandom::new();
        let mut bytes = [0_u8; BLIND_LEN];
        rng.fill(&mut bytes).unwrap();
        Self(bytes)
    }
}

impl AsRef<[u8]> for Blind {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&ChainItemInput> for Data {
    fn from(input: &ChainItemInput) -> Self {
        input.data.clone()
    }
}

impl From<&ChainItemInput> for Nonce {
    fn from(input: &ChainItemInput) -> Self {
        let data_hash: DataHash = (&input.data).into();
        let mut bytes = vec![];
        bytes.extend(data_hash.as_ref());
        bytes.extend(input.blind.as_ref());
        Self((&bytes).into())
    }
}

pub struct ChainItemInput {
    pub blind: Blind,
    pub data: Data,
    pub server: ecosystem::server::Server,
}

impl From<&ChainItemInput> for ecosystem::server::public_key::Key {
    fn from(input: &ChainItemInput) -> Self {
        *input.server.as_ref()
    }
}

impl From<&ChainItemInput> for Blind {
    fn from(input: &ChainItemInput) -> Self {
        input.blind
    }
}

impl std::convert::TryFrom<&ChainItemInput> for std::net::SocketAddr {
    type Error = error::ProvenanceError;
    fn try_from(input: &ChainItemInput) -> Result<Self, Self::Error> {
        let url: ecosystem::server::address::Url = (&input.server).try_into()?;
        match url.as_ref().to_socket_addrs()?.next() {
            Some(v) => Ok(v),
            None => Err(error::ProvenanceError::NoSocketAddrs(
                url.as_ref().to_string(),
            )),
        }
    }
}

impl std::convert::TryFrom<&ChainItemInput> for ChainItem {
    type Error = error::ProvenanceError;
    fn try_from(input: &ChainItemInput) -> Result<Self, Self::Error> {
        let addr: std::net::SocketAddr = input.try_into()?;
        let mut socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("Couldn't open UDP socket");
        let request: client::Request = input.try_into()?;

        socket.send_to(request.as_ref(), addr)?;

        let response: roughenough::RtMessage = client::receive_response(&mut socket)?;

        Ok(ChainItem {
            blind: input.into(),
            data: input.into(),
            pub_key: input.into(),
            response: (&response).try_into()?,
        })
    }
}
