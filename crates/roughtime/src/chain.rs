// the blind to make the hash unpredictable for the server
const BLIND_LEN: usize = humm_provenance_crypto::sha512::SHA512_OUTPUT_LEN;
#[derive(Clone, Copy)]
pub struct Blind([u8; BLIND_LEN]);

impl serde::Serialize for Blind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

// impl Default for Blind {
//     fn default() -> Self {
//         let rng = ring::rand::SystemRandom::new();
//         let mut bytes = [0_u8; BLIND_LEN];
//         rng.fill(&mut bytes).unwrap();
//         Self(bytes)
//     }
// }

impl AsRef<[u8]> for Blind {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub struct EncodedRtMessage(Vec<u8>);

impl AsRef<[u8]> for EncodedRtMessage {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl serde::Serialize for EncodedRtMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}

#[derive(serde::Serialize)]
pub struct ChainItem {
    blind: Blind,
    data: Data,
    pub_key: crate::ecosystem::server::public_key::Key,
    pub response: EncodedRtMessage,
}

impl From<&ChainItem> for Data {
    fn from(chain_item: &ChainItem) -> Self {
        chain_item.data.clone()
    }
}

impl From<&EncodedRtMessage> for Data {
    fn from(encoded_message: &EncodedRtMessage) -> Self {
        Self(encoded_message.as_ref().to_vec())
    }
}

#[derive(serde::Serialize)]
pub struct Chain(Vec<ChainItem>);

impl From<Vec<ChainItem>> for Chain {
    fn from(v: Vec<ChainItem>) -> Self {
        Self(v)
    }
}

impl From<Vec<u8>> for Data {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

// the input data for a chain item
// arbitrary bytes of binary data
#[derive(Clone)]
pub struct Data(Vec<u8>);

impl serde::Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self))
    }
}
