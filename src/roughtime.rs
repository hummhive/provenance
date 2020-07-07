pub mod ecosystem;
pub mod client;
pub mod chain;
use crate::error;
// use crate::crypto;
use crate::device;
use ecosystem::server::public_key;
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

impl std::convert::TryFrom<&RoughtimeInput> for chain::Chain {
    type Error = error::ProvenanceError;
    fn try_from(input: &RoughtimeInput) -> Result<Self, Self::Error> {
        let rng = &mut rand::thread_rng();
        // just keep looping until we get some good data
        // will fail in an infinite loop kind of way if:
        // - there is a bug
        // - there are network issues
        // - there are insufficient working servers to create a chain
        Ok(loop {
            let mut chain_items: Vec<chain::ChainItem> = vec![];
            let mut data: chain::Data = input.device_signature.as_ref().to_vec().into();
            // get some random servers from the ecosystem
            for server in input.ecosystem.as_ref().as_ref().choose_multiple(rng, DESIRED_CHAIN_LEN) {
                // try to build a chain without errors
                let chain_item: chain::ChainItem =
                    match (&chain::ChainItemInput {
                        blind: chain::Blind::default(),
                        data,
                        server: (*server).clone(),
                    }).try_into() {
                        Ok(v) => v,
                        Err(_) => break,
                    };
                data = (&chain_item.response).into();
                chain_items.push(chain_item);
            }
            if chain_items.len() == DESIRED_CHAIN_LEN {
                break chain_items.into();
            } else {
                // there was an error
                // keep looping
            }
        })
    }
}
