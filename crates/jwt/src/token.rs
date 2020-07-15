use humm_provenance_device as device;

#[derive(serde::Serialize)]
pub struct ProvenanceClaims {
    time_keys_hash: humm_provenance_roughtime::ecosystem::server::public_key::KeysHash,
    #[serde(rename = "aud")]
    audience: device::keys::PubKey,
}

#[derive(serde::Serialize)]
pub struct Token(pub(crate) String);
