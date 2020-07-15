pub struct SignatureInput {
    pub device_keypair: keys::Keypair,
    pub version_int: version::VersionInt,
    pub content_hash: content::Hash,
    pub jwt_signature: crypto::jwt::Signature,
}

impl TryFrom<&SignatureInput> for Signature {
    type Error = error::DeviceError;
    fn try_from(signature_input: &SignatureInput) -> Result<Self, Self::Error> {
        let mut to_sign: Vec<u8> = vec![];

        to_sign.extend(vec![<u8>::from(&signature_input.version_int)]);
        to_sign.extend(<[u8; SHA512_OUTPUT_LEN]>::from(&signature_input.content_hash).iter());
        to_sign.extend(
            <[u8; ed25519_dalek::SIGNATURE_LENGTH]>::from(&signature_input.jwt_signature).iter(),
        );

        Ok(Self(
            (&crypto::ed25519::signature::Ed25519SignatureInput {
                keypair: (&signature_input.device_keypair).into(),
                to_sign,
            })
                .try_into()?,
        ))
    }
}
