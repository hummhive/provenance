use crate::crypto;
use crate::device;
use crate::error;
use crate::jwt;
use crate::roughtime;
use jwt_compact::AlgorithmExt;

#[derive(serde::Serialize)]
pub struct ProvenanceClaims {
    time_keys_hash: roughtime::ecosystem::server::public_key::KeysHash,
    #[serde(rename = "aud")]
    audience: device::keys::PubKey,
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
        Ok(Self(
            jwt_compact::alg::Ed25519.token(header, &claims, &keypair)?,
        ))
    }
}

pub struct Signature(crypto::Ed25519Signature);

impl std::convert::TryFrom<&Token> for Signature {
    type Error = error::ProvenanceError;
    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        // need to get at signature directly due to missing upstream API
        // https://github.com/slowli/jwt-compact/issues/5
        // https://docs.rs/jwt-compact/0.2.0/src/jwt_compact/lib.rs.html#494
        let token_parts: Vec<_> = token.0.as_str().splitn(4, '.').collect();
        match &token_parts[..] {
            [_, _, signature] => {
                let mut decoded_signature = vec![0; 3 * (signature.len() + 3) / 4];
                let signature_len = base64::decode_config_slice(
                    signature,
                    base64::URL_SAFE_NO_PAD,
                    &mut decoded_signature[..],
                )?;
                decoded_signature.truncate(signature_len);
                let mut signature_bytes = [0; ed25519_dalek::SIGNATURE_LENGTH];
                signature_bytes.copy_from_slice(&decoded_signature[..]);
                Ok(Self(crypto::Ed25519Signature::from(signature_bytes)))
            }
            _ => Err(jwt_compact::ParseError::InvalidTokenStructure)?,
        }
    }
}

impl From<&Signature> for [u8; ed25519_dalek::SIGNATURE_LENGTH] {
    fn from(signature: &Signature) -> Self {
        (&signature.0).into()
    }
}
