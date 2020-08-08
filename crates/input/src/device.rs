use humm_jwt as jwt;
use humm_provenance_content as content;
use humm_crypto::sha512::SHA512_OUTPUT_LEN;
use humm_provenance_device as device;
use humm_provenance_version as version;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct SignatureInput {
    pub device_keypair: device::keys::Keypair,
    pub version_int: version::VersionInt,
    pub content_hash: content::Hash,
    pub jwt_signature: jwt::signature::Signature,
}

impl TryFrom<&SignatureInput> for device::signature::Signature {
    type Error = humm_crypto::error::CryptoError;
    fn try_from(signature_input: &SignatureInput) -> Result<Self, Self::Error> {
        let mut to_sign: Vec<u8> = vec![];

        to_sign.extend(vec![<u8>::from(&signature_input.version_int)]);
        to_sign.extend(<[u8; SHA512_OUTPUT_LEN]>::from(&signature_input.content_hash).iter());
        to_sign.extend(
            <[u8; ed25519_dalek::SIGNATURE_LENGTH]>::from(&signature_input.jwt_signature).iter(),
        );

        let ed25519_sig: humm_crypto::ed25519::signature::Ed25519Signature =
            (&crate::ed25519::signature::Ed25519SignatureInput {
                keypair: (&signature_input.device_keypair).into(),
                to_sign,
            })
                .try_into()?;

        Ok(Self::from(ed25519_sig))
    }
}
