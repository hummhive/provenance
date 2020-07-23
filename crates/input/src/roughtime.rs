use humm_provenance_device as device;
use humm_provenance_roughtime as roughtime;
use std::convert::TryInto;
pub mod chain;
pub mod client;
pub mod crypto;
use rand::seq::SliceRandom;

const DESIRED_CHAIN_LEN: usize = 3;

#[derive(serde::Serialize)]
pub struct Roughtime {
    pub public_keys: roughtime::ecosystem::server::public_key::Keys,
    pub chain: roughtime::chain::Chain,
}

pub struct RoughtimeInput {
    pub ecosystem: roughtime::ecosystem::Ecosystem,
    pub device_signature: device::signature::Signature,
}

impl std::convert::TryFrom<&RoughtimeInput> for roughtime::chain::Chain {
    type Error = roughtime::error::RoughtimeError;
    fn try_from(input: &RoughtimeInput) -> Result<Self, Self::Error> {
        let rng = &mut rand::thread_rng();
        // just keep looping until we get some good data
        // will fail in an infinite loop kind of way if:
        // - there is a bug
        // - there are network issues
        // - there are insufficient working servers to create a chain
        Ok(loop {
            let mut chain_items: Vec<roughtime::chain::ChainItem> = vec![];
            let mut data: roughtime::chain::Data = input.device_signature.as_ref().to_vec().into();
            // get some random servers from the ecosystem
            for server in input
                .ecosystem
                .as_ref()
                .as_ref()
                .choose_multiple(rng, DESIRED_CHAIN_LEN)
            {
                // try to build a chain without errors
                let chain_item: roughtime::chain::ChainItem =
                    match (&crate::roughtime::chain::ChainItemInput {
                        blind: roughtime::chain::Blind::default(),
                        data,
                        server: (*server).clone(),
                    })
                        .try_into()
                    {
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

impl std::convert::TryFrom<RoughtimeInput> for Roughtime {
    type Error = roughtime::error::RoughtimeError;
    fn try_from(input: RoughtimeInput) -> Result<Self, Self::Error> {
        Ok(Roughtime {
            public_keys: (&input.ecosystem).into(),
            chain: (&input).try_into()?,
        })
    }
}
