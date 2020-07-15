#[derive(serde::Serialize, Clone, Copy)]
pub enum Version {
    #[serde(rename = "0")]
    Zero,
}

pub struct VersionInt(u8);

impl From<&Version> for VersionInt {
    fn from(version: &Version) -> Self {
        Self(match version {
            Version::Zero => 0,
        })
    }
}

impl From<&VersionInt> for u8 {
    fn from(version_int: &VersionInt) -> Self {
        version_int.0
    }
}
