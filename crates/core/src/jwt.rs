pub struct JwtInput {
    pub device_pub_key: device::keys::PubKey,
    pub idp_keypair: idp::Keypair,
    pub time_keys_hash: roughtime::ecosystem::server::public_key::KeysHash,
}

impl From<&crate::JwtInput> for Keypair {
    fn from(input: &crate::JwtInput) -> Self {
        input.idp_keypair
    }
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

impl From<&crate::JwtInput> for ProvenanceClaims {
    fn from(input: &crate::JwtInput) -> Self {
        Self {
            time_keys_hash: input.into(),
            audience: input.into(),
        }
    }
}

impl std::convert::TryFrom<&crate::JwtInput> for Token {
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
