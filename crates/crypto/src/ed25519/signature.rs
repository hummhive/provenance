use crate::ed25519::keypair::Ed25519Keypair;
use crate::sha512::SHA512_OUTPUT_LEN;
use serde::de::Error;
use std::convert::TryInto;

#[derive(Clone, Copy)]
pub struct Ed25519Signature([u8; ed25519_dalek::SIGNATURE_LENGTH]);

#[cfg(test)]
#[test]
fn ed25519_signature_smoke() {
    assert_eq!(64, ed25519_dalek::SIGNATURE_LENGTH,);

    Ed25519Signature([0; ed25519_dalek::SIGNATURE_LENGTH]);
}

impl From<[u8; ed25519_dalek::SIGNATURE_LENGTH]> for Ed25519Signature {
    // if we have the literal byte array we accept it
    fn from(literal_bytes: [u8; ed25519_dalek::SIGNATURE_LENGTH]) -> Self {
        Self(literal_bytes)
    }
}

#[cfg(test)]
#[test]
fn ed25519_signature_from_byte_array() {
    let array = [0; ed25519_dalek::SIGNATURE_LENGTH];
    let signature = Ed25519Signature::from(array);

    assert_eq!(&array.to_vec(), &signature.as_ref().to_vec());
}

impl From<&ed25519_dalek::Signature> for Ed25519Signature {
    fn from(signature: &ed25519_dalek::Signature) -> Self {
        Self(signature.to_bytes())
    }
}

#[cfg(test)]
#[test]
fn ed25519_signature_from_dalek() {
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signature: ed25519_dalek::Signature = keypair.sign(message);

    let crypto_signature = Ed25519Signature::from(&signature);

    assert_eq!(
        crypto_signature.as_ref().to_vec(),
        signature.to_bytes().to_vec()
    );
}

impl From<&Ed25519Signature> for [u8; ed25519_dalek::SIGNATURE_LENGTH] {
    fn from(signature: &Ed25519Signature) -> Self {
        signature.0
    }
}

#[cfg(test)]
#[test]
fn ed25519_signature_to_array() {
    let inner = [0; ed25519_dalek::SIGNATURE_LENGTH];
    let signature = Ed25519Signature::from(inner);

    assert_eq!(
        inner.to_vec(),
        <[u8; ed25519_dalek::SIGNATURE_LENGTH]>::from(&signature).to_vec(),
    );
}

impl std::convert::AsRef<[u8]> for Ed25519Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn ed25519_signature_as_ref() {
    let inner = [0; ed25519_dalek::SIGNATURE_LENGTH];
    let signature = Ed25519Signature::from(inner);

    assert_eq!(inner.to_vec(), signature.as_ref().to_vec(),);
}

impl serde::ser::Serialize for Ed25519Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

impl<'de> serde::de::Deserialize<'de> for Ed25519Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
        let bytes: &[u8] = &base64::decode(s).map_err(|_| {
            D::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"a string that is not base64 bytes",
            )
        })?[..];
        if bytes.len() != SHA512_OUTPUT_LEN {
            return Err(D::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"incorrect length public key",
            ));
        }
        let mut inner: [u8; SHA512_OUTPUT_LEN] = [0; SHA512_OUTPUT_LEN];
        inner.clone_from_slice(bytes);
        Ok(Self(inner))
    }
}

#[cfg(test)]
#[test]
fn ed25519_signature_serde() {
    let signature = Ed25519Signature::from([0; ed25519_dalek::SIGNATURE_LENGTH]);
    let s = serde_json::to_string(&signature).unwrap();
    assert_eq!("\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==\"", s);

    let restore: Ed25519Signature = serde_json::from_str(&s).unwrap();
    assert_eq!(signature.as_ref().to_vec(), restore.as_ref().to_vec());
}

pub struct Ed25519SignatureInput {
    pub keypair: Ed25519Keypair,
    pub to_sign: Vec<u8>,
}

impl std::convert::TryFrom<&Ed25519SignatureInput> for Ed25519Signature {
    type Error = crate::error::CryptoError;
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
