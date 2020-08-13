use humm_crypto::sha512::Sha512Hash;
use humm_crypto::sha512::SHA512_OUTPUT_LEN;

pub struct Content(Vec<u8>);

#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct Hash(Sha512Hash);

impl From<Vec<u8>> for Content {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl From<&Content> for Hash {
    fn from(content: &Content) -> Self {
        Self((&content.0).into())
    }
}

impl From<&Hash> for [u8; SHA512_OUTPUT_LEN] {
    fn from(hash: &Hash) -> Self {
        (&hash.0).into()
    }
}

#[cfg(test)]
mod test {

    use super::Content;
    use super::Hash;

    #[test]
    fn content_hash() {
        let content = Content(b"foo".to_vec());
        let content_hash: Hash = (&content).into();

        assert_eq!(
            [
                0xf7, 0xfb, 0xba, 0x6e, 0x06, 0x36, 0xf8, 0x90, 0xe5, 0x6f, 0xbb, 0xf3, 0x28, 0x3e,
                0x52, 0x4c, 0x6f, 0xa3, 0x20, 0x4a, 0xe2, 0x98, 0x38, 0x2d, 0x62, 0x47, 0x41, 0xd0,
                0xdc, 0x66, 0x38, 0x32, 0x6e, 0x28, 0x2c, 0x41, 0xbe, 0x5e, 0x42, 0x54, 0xd8, 0x82,
                0x07, 0x72, 0xc5, 0x51, 0x8a, 0x2c, 0x5a, 0x8c, 0x0c, 0x7f, 0x7e, 0xda, 0x19, 0x59,
                0x4a, 0x7e, 0xb5, 0x39, 0x45, 0x3e, 0x1e, 0xd7
            ]
            .to_vec(),
            AsRef::<[u8]>::as_ref(&content_hash.0).to_vec()
        )
    }
}
