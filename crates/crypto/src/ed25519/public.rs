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

impl From<&ed25519_dalek::Keypair> for Ed25519PubKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self::from(keypair.public.to_bytes())
    }
}

#[cfg(test)]
#[test]
fn ed25519pub_key_from_dalek_keypair() {
    let mut csprng = rand::rngs::OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    let pubkey = crate::ed25519::public::Ed25519PubKey::from(&keypair);

    assert_eq!(keypair.public.to_bytes(), pubkey.as_ref(),);
}
