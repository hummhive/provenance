use crate::error;
use humm_crypto as crypto;
use std::convert::TryFrom;

/// the only key type currently used in ecosystem.json is ed25519
#[derive(PartialEq, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub(crate) enum KeyType {
    #[serde(rename = "ed25519")]
    Ed25519,
}

#[cfg(test)]
#[test]
fn key_type_serde() {
    let key_type = KeyType::Ed25519;
    let s = serde_json::to_string(&key_type).unwrap();

    assert_eq!(r#""ed25519""#, s,);

    let r: KeyType = serde_json::from_str(&s).unwrap();

    assert_eq!(r, key_type,);
}

/// public keys in an ecosystem.json are 32 bytes as base64
#[derive(PartialEq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Key(crypto::ed25519::public::Ed25519PubKey);

impl From<&Key> for [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] {
    fn from(key: &Key) -> Self {
        (&key.0).into()
    }
}

impl From<[u8; ed25519_dalek::PUBLIC_KEY_LENGTH]> for Key {
    fn from(arr: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH]) -> Self {
        Self(arr.into())
    }
}

#[cfg(test)]
#[test]
fn key_serde() {
    let key = Key::from([5; ed25519_dalek::PUBLIC_KEY_LENGTH]);
    let s = serde_json::to_string(&key).unwrap();

    assert_eq!(r#""BQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQU=""#, s);

    let arr: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = (&key).into();
    assert_eq!(vec![5; ed25519_dalek::PUBLIC_KEY_LENGTH], arr.to_vec());

    let r: Key = serde_json::from_str(&s).unwrap();

    assert_eq!(r, key);
}

#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct Keys(Vec<Key>);

impl From<Vec<Key>> for Keys {
    fn from(keys: Vec<Key>) -> Self {
        Self(keys)
    }
}

impl AsRef<[Key]> for Keys {
    fn as_ref(&self) -> &[Key] {
        self.0.as_ref()
    }
}

pub struct KeysPortable(String);

impl From<String> for KeysPortable {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for KeysPortable {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

#[cfg(test)]
#[test]
fn keys_portable_serde() {
    let keys = Keys::from(vec![
        Key::from([5; ed25519_dalek::PUBLIC_KEY_LENGTH]),
        Key::from([9; ed25519_dalek::PUBLIC_KEY_LENGTH]),
    ]);

    let keys_portable = KeysPortable::try_from(&keys).unwrap();

    assert_eq!(
        r#"["BQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQU=","CQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQk="]"#,
        keys_portable.0,
    );

    let round = Keys::try_from(&keys_portable).unwrap();

    assert_eq!(round.as_ref(), keys.as_ref(),);
}

impl TryFrom<&Keys> for KeysPortable {
    type Error = error::RoughtimeError;
    fn try_from(keys: &Keys) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::to_string(keys)?))
    }
}

impl TryFrom<&KeysPortable> for Keys {
    type Error = error::RoughtimeError;
    fn try_from(keys_portable: &KeysPortable) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::from_str(&keys_portable.0)?))
    }
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct KeysHash(crypto::sha512::Sha512Hash);

impl From<&Keys> for KeysHash {
    fn from(keys: &Keys) -> Self {
        let mut bytes: Vec<u8> = vec![];
        for key in &keys.0 {
            let key_bytes: [u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = key.into();
            bytes.extend(key_bytes.iter());
        }
        Self((&bytes).into())
    }
}

impl AsRef<[u8]> for KeysHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
#[test]
fn keys_to_keys_hash() {
    let keys = Keys::from(vec![
        Key::from([5; ed25519_dalek::PUBLIC_KEY_LENGTH]),
        Key::from([9; ed25519_dalek::PUBLIC_KEY_LENGTH]),
    ]);

    let keys_hash = KeysHash::from(&keys);

    let s = serde_json::to_string(&keys_hash).unwrap();

    // hash of 5's and 9's cross referenced against dalek and ring
    // @see https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=40c0671da85d1d71544b3c9facffc541
    assert_eq!(
        r#""WFBm2ppnsAJelUxmIpkMmUCjC0bZlSWyYXft3jUlGbcrvHZzMsgBuhMRPlggcB8ZLX1PZP2o63hCtNQp0cCw+w==""#,
        s,
    );

    let bytes: KeysHash = serde_json::from_str(&s).unwrap();

    assert_eq!(
        vec![
            88, 80, 102, 218, 154, 103, 176, 2, 94, 149, 76, 102, 34, 153, 12, 153, 64, 163, 11,
            70, 217, 149, 37, 178, 97, 119, 237, 222, 53, 37, 25, 183, 43, 188, 118, 115, 50, 200,
            1, 186, 19, 17, 62, 88, 32, 112, 31, 25, 45, 125, 79, 100, 253, 168, 235, 120, 66, 180,
            212, 41, 209, 192, 176, 251
        ],
        bytes.as_ref(),
    );
}
