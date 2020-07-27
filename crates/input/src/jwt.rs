use humm_provenance_device as device;
use humm_provenance_jwt as jwt;
use humm_provenance_roughtime as roughtime;
use jwt_compact::AlgorithmExt;
use std::convert::TryInto;

pub struct JwtInput {
    pub device_pub_key: device::keys::PubKey,
    pub idp_keypair: jwt::idp::Keypair,
    pub time_keys_hash: roughtime::ecosystem::server::public_key::KeysHash,
}

impl From<&JwtInput> for jwt::idp::Keypair {
    fn from(input: &JwtInput) -> Self {
        input.idp_keypair
    }
}

impl From<&JwtInput> for device::keys::PubKey {
    fn from(input: &JwtInput) -> Self {
        input.device_pub_key
    }
}

impl std::convert::TryFrom<&JwtInput> for jwt::idp::PubKey {
    type Error = jwt::error::JwtError;
    fn try_from(input: &JwtInput) -> Result<Self, Self::Error> {
        Ok(Self::try_from(&input.idp_keypair)?)
    }
}

impl std::convert::TryFrom<&JwtInput> for jwt::Jwt {
    type Error = jwt::error::JwtError;
    fn try_from(input: &JwtInput) -> Result<Self, Self::Error> {
        Ok(Self {
            token: jwt::token::Token::try_from(input)?.into(),
            idp_pub_key: input.try_into()?,
        })
    }
}

impl From<&JwtInput> for roughtime::ecosystem::server::public_key::KeysHash {
    fn from(input: &JwtInput) -> Self {
        input.time_keys_hash
    }
}

impl From<&JwtInput> for jwt::token::ProvenanceClaims {
    fn from(input: &JwtInput) -> Self {
        Self::from((input.into(), input.into()))
    }
}

impl std::convert::TryFrom<&JwtInput> for jwt::token::Token {
    type Error = jwt::error::JwtError;
    fn try_from(input: &JwtInput) -> Result<Self, Self::Error> {
        let header = jwt_compact::Header {
            ..Default::default()
        };
        let claims = jwt_compact::Claims::new(jwt::token::ProvenanceClaims::from(input))
            .set_duration_and_issuance(chrono::Duration::minutes(10))
            .set_not_before(chrono::Utc::now() - chrono::Duration::minutes(10));

        let keypair = ed25519_dalek::Keypair::try_from(&input.idp_keypair)?;
        Ok(Self::from(
            jwt_compact::alg::Ed25519.token(header, &claims, &keypair)?,
        ))
    }
}
