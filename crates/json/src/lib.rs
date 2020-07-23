pub mod error;

#[derive(Debug)]
pub struct Json(String);

impl std::convert::TryFrom<String> for Json {
    type Error = error::ProvenanceError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        // ensure s is valid json
        // https://users.rust-lang.org/t/serde-json-checking-syntax-of-json-file/16265/4?u=thedavidmeister
        let _: serde::de::IgnoredAny = serde_json::from_str(&s)?;
        Ok(Json(s))
    }
}

impl From<Json> for String {
    fn from(json: Json) -> Self {
        json.0
    }
}
