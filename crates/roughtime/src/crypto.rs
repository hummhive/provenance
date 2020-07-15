use crate::chain::Data;

pub struct Nonce(humm_provenance_crypto::sha512::Sha512Hash);

impl AsRef<[u8]> for Nonce {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

pub struct DataHash(humm_provenance_crypto::sha512::Sha512Hash);

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
