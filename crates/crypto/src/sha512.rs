/// replica of ring::digest::SHA512_OUTPUT_LEN
pub const SHA512_OUTPUT_LEN: usize = 512 / 8;

#[derive(Clone, Copy)]
pub struct Sha512Hash([u8; SHA512_OUTPUT_LEN]);

impl serde::ser::Serialize for Sha512Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

impl From<&Sha512Hash> for [u8; SHA512_OUTPUT_LEN] {
    fn from(hash: &Sha512Hash) -> Self {
        hash.0
    }
}

impl std::convert::AsRef<[u8]> for Sha512Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
