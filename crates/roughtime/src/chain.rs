#[derive(serde::Serialize)]
pub struct ChainItem {
    blind: Blind,
    data: Data,
    pub_key: server::public_key::Key,
    pub response: client::EncodedRtMessage,
}

impl From<&ChainItem> for Data {
    fn from(chain_item: &ChainItem) -> Self {
        chain_item.data.clone()
    }
}

impl From<&client::EncodedRtMessage> for Data {
    fn from(encoded_message: &client::EncodedRtMessage) -> Self {
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
