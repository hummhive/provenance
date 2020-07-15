use serde::de::Error;

/// matches ring::digest::SHA512_OUTPUT_LEN
pub const SHA512_OUTPUT_LEN: usize = 512 / std::mem::size_of::<u64>();

#[derive(Clone, Copy)]
pub struct Sha512Hash([u8; SHA512_OUTPUT_LEN]);

#[cfg(test)]
#[test]
fn sha512_smoke() {
    assert_eq!(64, SHA512_OUTPUT_LEN,);

    Sha512Hash([0; SHA512_OUTPUT_LEN]);
}

impl From<[u8; SHA512_OUTPUT_LEN]> for Sha512Hash {
    fn from(bytes: [u8; SHA512_OUTPUT_LEN]) -> Self {
        Self(bytes)
    }
}

#[cfg(test)]
#[test]
fn sha512_from_byte_array() {
    let array = [0; SHA512_OUTPUT_LEN];
    let sha512 = Sha512Hash::from(array);

    assert_eq!(&array.to_vec(), &sha512.as_ref().to_vec(),);
}

impl From<&Vec<u8>> for Sha512Hash {
    fn from(vec: &Vec<u8>) -> Self {
        let mut array = [0; SHA512_OUTPUT_LEN];
        array.clone_from_slice(vec);

        Self::from(array)
    }
}

impl serde::ser::Serialize for Sha512Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

impl<'de> serde::de::Deserialize<'de> for Sha512Hash {
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
        Ok(Sha512Hash(inner))
    }
}

#[cfg(test)]
#[test]
fn sha512_serde() {
    let sha512 = Sha512Hash::from([0; SHA512_OUTPUT_LEN]);
    let s = serde_json::to_string(&sha512).unwrap();
    assert_eq!("\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==\"", s);

    let restore: Sha512Hash = serde_json::from_str(&s).unwrap();
    assert_eq!(sha512.as_ref().to_vec(), restore.as_ref().to_vec());
}

impl From<&Sha512Hash> for [u8; SHA512_OUTPUT_LEN] {
    fn from(hash: &Sha512Hash) -> Self {
        hash.0
    }
}

#[cfg(test)]
#[test]
fn sha512_to_array() {
    let inner = [0; SHA512_OUTPUT_LEN];
    let sha512 = Sha512Hash::from(inner);

    assert_eq!(
        inner.to_vec(),
        <[u8; SHA512_OUTPUT_LEN]>::from(&sha512).to_vec()
    );
}

impl std::convert::AsRef<[u8]> for Sha512Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn sha512_as_ref() {
    let inner = [0; SHA512_OUTPUT_LEN];
    let sha512 = Sha512Hash::from(inner);

    assert_eq!(inner.to_vec(), sha512.as_ref().to_vec());
}
