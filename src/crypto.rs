use serde::ser;
use serde::de;
use serde::de::Error;
use std::convert::TryInto;

pub struct Sha512Hash([u8; ring::digest::SHA512_OUTPUT_LEN]);

impl ser::Serialize for Sha512Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

impl From<&Vec<u8>> for Sha512Hash {
    fn from(bytes: &Vec<u8>) -> Self {
        let mut hash = [0; ring::digest::SHA512_OUTPUT_LEN];
            hash.copy_from_slice(&
            ring::digest::digest(&ring::digest::SHA512, &bytes).as_ref().to_owned()
        );
        Self(hash)
    }
}

// impl std::convert::AsRef<[u8; ring::digest::SHA512_OUTPUT_LEN]> for Sha512Hash {
//     fn as_ref(&self) -> &[u8; ring::digest::SHA512_OUTPUT_LEN] {
//         &self.0
//     }
// }

impl std::convert::AsRef<[u8]> for Sha512Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ed25519Key([u8; 32]);

impl std::convert::AsRef<[u8]> for Ed25519Key {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl <'de> de::Deserialize<'de> for Ed25519Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
    D: de::Deserializer<'de>,
    {
        let s: &str = de::Deserialize::deserialize(deserializer)?;
        let bytes: &[u8] = &base64::decode(s).map_err(|_| D::Error::invalid_value(de::Unexpected::Str(s), &"a string that is not base64 bytes"))?[..];
        Ok(Ed25519Key(bytes.try_into().map_err(|_| D::Error::invalid_value(de::Unexpected::Str(s), &"incorrect length public key"))?))
    }
}

impl ser::Serialize for Ed25519Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}
