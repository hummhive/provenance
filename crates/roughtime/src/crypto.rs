use crate::chain::Data;

pub struct Nonce(humm_crypto::sha512::Sha512Hash);

impl From<humm_crypto::sha512::Sha512Hash> for Nonce {
    fn from(sha512: humm_crypto::sha512::Sha512Hash) -> Self {
        Self(sha512)
    }
}

impl AsRef<[u8]> for Nonce {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

pub struct DataHash(humm_crypto::sha512::Sha512Hash);

impl From<&crate::chain::Data> for DataHash {
    fn from(data: &Data) -> Self {
        Self((&data.as_ref().to_vec()).into())
    }
}

impl AsRef<[u8]> for DataHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
