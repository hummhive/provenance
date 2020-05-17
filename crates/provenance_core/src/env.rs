use super::error;

/// the key of the environment variable
/// used to lookup val from the environment
pub struct Key(String);

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<Key> for String {
    fn from(k: Key) -> Self {
        k.0
    }
}

/// the value of an environment variable
/// may not be set in the current environment
pub struct Val(String);

impl From<String> for Val {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Val {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<Val> for String {
    fn from(v: Val) -> Self {
        v.0
    }
}

pub trait Var: Into<Key> + std::convert::TryInto<Val, Error=error::CoreError> { }
