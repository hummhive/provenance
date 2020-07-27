use device::keys::PubKey;
use humm_provenance_device as device;
use humm_provenance_roughtime::ecosystem::server::public_key::KeysHash;

#[derive(serde::Serialize)]
pub struct ProvenanceClaims {
    time_keys_hash: KeysHash,
    #[serde(rename = "aud")]
    audience: PubKey,
}

impl From<(KeysHash, PubKey)> for ProvenanceClaims {
    fn from((time_keys_hash, audience): (KeysHash, PubKey)) -> Self {
        Self {
            time_keys_hash,
            audience,
        }
    }
}

#[derive(serde::Serialize)]
pub struct Token(String);

impl From<String> for Token {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Token {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
