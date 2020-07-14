use crate::error;
// use crate::crypto;
use crate::device;
// use std::net::UdpSocket;
use rand::seq::SliceRandom;
use std::convert::TryInto;

const DESIRED_CHAIN_LEN: usize = 3;

#[derive(serde::Serialize)]
pub struct Roughtime {
    pub public_keys: public_key::Keys,
    pub chain: chain::Chain,
}

pub struct RoughtimeInput {
    pub ecosystem: ecosystem::Ecosystem,
    pub device_signature: device::signature::Signature,
}

impl std::convert::TryFrom<RoughtimeInput> for Roughtime {
    type Error = error::ProvenanceError;
    fn try_from(input: RoughtimeInput) -> Result<Self, Self::Error> {
        Ok(Roughtime {
            public_keys: (&input.ecosystem).into(),
            chain: (&input).try_into()?,
        })
    }
}
