use crate::error;
use device::keys::PubKey;
use humm_crypto as crypto;
use humm_provenance_device as device;
use humm_provenance_roughtime::ecosystem::server::public_key::KeysHash;
use jwt_compact::prelude::*;
use std::convert::TryFrom;

#[derive(serde::Serialize)]
pub struct ProvenanceClaims {
    time_keys_hash: KeysHash,
    #[serde(rename = "aud")]
    audience: PubKey,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NoClaims {}

impl From<(KeysHash, PubKey)> for ProvenanceClaims {
    fn from((time_keys_hash, audience): (KeysHash, PubKey)) -> Self {
        Self {
            time_keys_hash,
            audience,
        }
    }
}

pub struct Token(jwt_compact::SignedToken<jwt_compact::alg::Ed25519, NoClaims>);

impl TryFrom<(crypto::ed25519::public::Ed25519PubKey, String)> for Token {
    type Error = error::JwtError;
    fn try_from(
        (pubkey, s): (crypto::ed25519::public::Ed25519PubKey, String),
    ) -> Result<Self, Self::Error> {
        Self::try_from((pubkey, s.as_str()))
    }
}

impl TryFrom<(crypto::ed25519::public::Ed25519PubKey, &str)> for Token {
    type Error = error::JwtError;
    fn try_from(
        (pubkey, s): (crypto::ed25519::public::Ed25519PubKey, &str),
    ) -> Result<Self, Self::Error> {
        let untrusted = UntrustedToken::try_from(s)?;
        let signed_token: jwt_compact::SignedToken<jwt_compact::alg::Ed25519, NoClaims> =
            jwt_compact::alg::Ed25519.validate_for_signed_token(
                &untrusted,
                &jwt_compact::alg::Ed25519VerifyingKey::from_slice(pubkey.as_ref())?,
            )?;
        signed_token
            .token
            .claims()
            .validate_expiration(TimeOptions::default())?;
        // .validate_expiration(TimeOptions {
        //     current_time: Some(chrono::DateTime::<chrono::Utc>::from_utc(
        //         chrono::NaiveDateTime::from_timestamp(1_000_000_000_000, 0),
        //         chrono::Utc,
        //     )),
        //     ..Default::default()
        // })?;
        Ok(Self(signed_token))
    }
}

impl Token {
    pub fn signed_token(&self) -> &jwt_compact::SignedToken<jwt_compact::alg::Ed25519, NoClaims> {
        &self.0
    }
}
