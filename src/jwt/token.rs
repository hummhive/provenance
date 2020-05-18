use crate::roughtime;
use crate::device;
use crate::error;
use crate::jwt;
use jwt_compact::AlgorithmExt;

#[derive(serde::Serialize)]
pub struct ProvenanceClaims {
    time_keys_hash: roughtime::ecosystem::server::public_key::KeysHash,
    #[serde(rename="aud")]
    audience: device::PubKey,
}

#[derive(serde::Serialize)]
pub struct Token(String);

impl From<&jwt::JwtInput> for ProvenanceClaims {
    fn from(input: &jwt::JwtInput) -> Self {
        Self {
            time_keys_hash: input.into(),
            audience: input.into(),
        }
    }
}

impl std::convert::TryFrom<&jwt::JwtInput> for Token {
    type Error = error::ProvenanceError;
    fn try_from(input: &jwt::JwtInput) -> Result<Self, Self::Error> {
        let header = jwt_compact::Header {
            ..Default::default()
        };
        let claims = jwt_compact::Claims::new(ProvenanceClaims::from(input))
            .set_duration_and_issuance(chrono::Duration::minutes(10))
            .set_not_before(chrono::Utc::now() - chrono::Duration::minutes(10));

        let keypair = ed25519_dalek::Keypair::try_from(&input.idp_keypair)?;
        Ok(Self(jwt_compact::alg::Ed25519.token(header, &claims, &keypair)?))
    }
}
