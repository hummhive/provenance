use crate::error;

pub struct Signature(humm_crypto::ed25519::signature::Ed25519Signature);

impl std::convert::TryFrom<&crate::token::Token> for Signature {
    type Error = error::JwtError;
    fn try_from(token: &crate::token::Token) -> Result<Self, Self::Error> {
        // need to get at signature directly due to missing upstream API
        // https://github.com/slowli/jwt-compact/issues/5
        // https://docs.rs/jwt-compact/0.2.0/src/jwt_compact/lib.rs.html#494
        let token_parts: Vec<_> = token.as_str().splitn(4, '.').collect();
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
                Ok(Self(
                    humm_crypto::ed25519::signature::Ed25519Signature::from(
                        signature_bytes,
                    ),
                ))
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
