use humm_crypto::ed25519::keypair::Ed25519Keypair;
use humm_crypto::ed25519::signature::Ed25519Signature;
use std::convert::TryInto;

pub struct Ed25519SignatureInput {
    pub keypair: Ed25519Keypair,
    pub to_sign: Vec<u8>,
}

impl std::convert::TryFrom<&Ed25519SignatureInput> for Ed25519Signature {
    type Error = humm_crypto::error::CryptoError;
    fn try_from(signature_input: &Ed25519SignatureInput) -> Result<Self, Self::Error> {
        let lib_keypair: ed25519_dalek::Keypair = (&signature_input.keypair).try_into()?;
        Ok((&lib_keypair.sign(&signature_input.to_sign)).into())
    }
}

#[cfg(test)]
#[test]
fn ed25519_signature_input() {
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let to_sign: &[u8] = b"This is a test of the tsunami alert system.";

    let signature_input = Ed25519SignatureInput {
        keypair: (&keypair).into(),
        to_sign: to_sign.to_vec(),
    };

    let signature: Ed25519Signature = (&signature_input).try_into().unwrap();

    assert_eq!(
        keypair.sign(to_sign).to_bytes().to_vec(),
        signature.as_ref().to_vec(),
    );
}
