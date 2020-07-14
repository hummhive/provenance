pub mod content;
pub mod device;
pub mod env;
pub mod error;
pub mod json;
pub mod jwt;
pub mod roughtime;
pub mod version;

#[derive(serde::Serialize)]
pub struct Provenance {
    pub version: version::Version,
    pub content_hash: content::Hash,
    pub roughtime: roughtime::Roughtime,
    pub jwt: jwt::Jwt,
    pub device_signature: device::signature::Signature,
}
