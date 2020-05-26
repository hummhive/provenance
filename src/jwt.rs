pub mod idp;
pub mod token;

use crate::roughtime;
use crate::device;
use crate::error;
use std::convert::TryInto;

pub struct JwtInput {
    pub device_pub_key: device::keys::PubKey,
    pub idp_keypair: idp::Keypair,
    pub time_keys_hash: roughtime::ecosystem::server::public_key::KeysHash,
}

impl From<&JwtInput> for device::keys::PubKey {
    fn from(input: &JwtInput) -> Self {
        input.device_pub_key
    }
}

impl std::convert::TryFrom<&JwtInput> for idp::PubKey {
    type Error = error::ProvenanceError;
    fn try_from(input: &JwtInput) -> Result<Self, Self::Error> {
        Ok(Self::try_from(&input.idp_keypair)?)
    }
}

impl std::convert::TryFrom<&JwtInput> for Jwt {
    type Error = error::ProvenanceError;
    fn try_from(input: &JwtInput) -> Result<Self, Self::Error> {
        Ok(Self {
            token: token::Token::try_from(input)?.into(),
            idp_pub_key: input.try_into()?,
        })
    }
}

impl From<&JwtInput> for roughtime::ecosystem::server::public_key::KeysHash {
    fn from(input: &JwtInput) -> Self {
        input.time_keys_hash
    }
}

#[derive(serde::Serialize)]
pub struct Jwt {
    pub idp_pub_key: idp::PubKey,
    pub token: token::Token,
}
