pub mod device;
pub mod ed25519;
pub mod json;
pub mod jwt;
pub mod roughtime;

use humm_provenance_content as content;
use humm_provenance_device;
use humm_provenance_version as version;

#[derive(serde::Serialize)]
pub struct Provenance {
    pub version: version::Version,
    pub content_hash: content::Hash,
    pub roughtime: roughtime::Roughtime,
    pub jwt: humm_provenance_jwt::Jwt,
    pub device_signature: humm_provenance_device::signature::Signature,
}
