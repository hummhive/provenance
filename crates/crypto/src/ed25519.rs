use serde::de::Error;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ed25519PubKey([u8; ed25519_dalek::PUBLIC_KEY_LENGTH]);

#[cfg(test)]
#[test]
fn ed25519pub_key_smoke() {
    assert_eq!(32, ed25519_dalek::PUBLIC_KEY_LENGTH,);

    Ed25519PubKey([0; ed25519_dalek::PUBLIC_KEY_LENGTH]);
}

impl From<&Ed25519PubKey> for [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] {
    fn from(pub_key: &Ed25519PubKey) -> Self {
        pub_key.0
    }
}

#[cfg(test)]
#[test]
fn ed25519pub_key_to_array() {
    let inner = [0; ed25519_dalek::PUBLIC_KEY_LENGTH];
    let pubkey = Ed25519PubKey(inner);

    assert_eq!(
        inner,
        <[u8; ed25519_dalek::PUBLIC_KEY_LENGTH]>::from(&pubkey),
    );
}

impl std::convert::AsRef<[u8]> for Ed25519PubKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn ed25519pub_key_as_ref() {
    let inner = [0; ed25519_dalek::PUBLIC_KEY_LENGTH];
    let pubkey = Ed25519PubKey(inner);

    assert_eq!(&inner, pubkey.as_ref(),);
}

impl From<&ed25519_dalek::Keypair> for Ed25519PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.public.to_bytes())
    }
}

#[cfg(test)]
#[test]
fn ed25519pub_key_from_dalek_keypair() {
    let mut csprng = rand::rngs::OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    let pubkey = Ed25519PubKey::from(&keypair);

    assert_eq!(keypair.public.to_bytes(), pubkey.as_ref(),);
}

impl From<[u8; ed25519_dalek::PUBLIC_KEY_LENGTH]> for Ed25519PubKey {
    fn from(bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH]) -> Self {
        Self(bytes)
    }
}

#[cfg(test)]
#[test]
fn ed25519pub_key_from_byte_array() {
    let array = [0; ed25519_dalek::PUBLIC_KEY_LENGTH];
    let pubkey = Ed25519PubKey::from(array);

    assert_eq!(&array, pubkey.as_ref(),);
}

impl<'de> serde::de::Deserialize<'de> for Ed25519PubKey {
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
        Ok(Ed25519PubKey(bytes.try_into().map_err(|_| {
            D::Error::invalid_value(
                serde::de::Unexpected::Str(s),
                &"incorrect length public key",
            )
        })?))
    }
}

impl serde::ser::Serialize for Ed25519PubKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

#[cfg(test)]
#[test]
fn ed25519pub_key_serde() {
    let pubkey = Ed25519PubKey::from([0; ed25519_dalek::PUBLIC_KEY_LENGTH]);
    let s = serde_json::to_string(&pubkey).unwrap();
    assert_eq!("\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=\"", s);

    assert_eq!(pubkey, serde_json::from_str(&s).unwrap(),);
}

pub struct Ed25519SecretKey([u8; ed25519_dalek::SECRET_KEY_LENGTH]);

impl From<&ed25519_dalek::Keypair> for Ed25519SecretKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.secret.to_bytes())
    }
}

#[derive(Clone, Copy)]
pub struct Ed25519Keypair([u8; ed25519_dalek::KEYPAIR_LENGTH]);

impl std::convert::TryFrom<&Ed25519Keypair> for Ed25519PubKey {
    type Error = crate::error::CryptoError;
    fn try_from(keypair: &Ed25519Keypair) -> Result<Self, Self::Error> {
        Ok(Self::from(&ed25519_dalek::Keypair::try_from(keypair)?))
    }
}

impl From<&ed25519_dalek::Keypair> for Ed25519Keypair {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.to_bytes())
    }
}

impl std::convert::TryFrom<&Ed25519Keypair> for ed25519_dalek::Keypair {
    type Error = crate::error::CryptoError;
    fn try_from(keypair: &Ed25519Keypair) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(&keypair.0)?)
    }
}

#[derive(Clone, Copy)]
pub struct Ed25519Signature([u8; ed25519_dalek::SIGNATURE_LENGTH]);

impl From<[u8; ed25519_dalek::SIGNATURE_LENGTH]> for Ed25519Signature {
    // if we have the literal byte array we accept it
    fn from(literal_bytes: [u8; ed25519_dalek::SIGNATURE_LENGTH]) -> Self {
        Self(literal_bytes)
    }
}

impl From<&ed25519_dalek::Signature> for Ed25519Signature {
    fn from(signature: &ed25519_dalek::Signature) -> Self {
        Self(signature.to_bytes())
    }
}

impl From<&Ed25519Signature> for [u8; ed25519_dalek::SIGNATURE_LENGTH] {
    fn from(signature: &Ed25519Signature) -> Self {
        signature.0
    }
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

impl std::convert::AsRef<[u8]> for Ed25519Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl serde::ser::Serialize for Ed25519Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}
