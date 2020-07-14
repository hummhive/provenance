pub struct Ed25519SecretKey([u8; ed25519_dalek::SECRET_KEY_LENGTH]);

#[cfg(test)]
#[test]
fn ed25519_secret_smoke() {
    assert_eq!(32, ed25519_dalek::SECRET_KEY_LENGTH,);

    Ed25519SecretKey([0; ed25519_dalek::SECRET_KEY_LENGTH]);
}

impl From<[u8; ed25519_dalek::SECRET_KEY_LENGTH]> for Ed25519SecretKey {
    fn from(bytes: [u8; ed25519_dalek::SECRET_KEY_LENGTH]) -> Self {
        Self(bytes)
    }
}

#[cfg(test)]
#[test]
fn ed25519_secret_from_byte_array() {
    let array = [0; ed25519_dalek::SECRET_KEY_LENGTH];
    let secret = Ed25519SecretKey::from(array);

    assert_eq!(&array, secret.as_ref(),);
}

impl std::convert::AsRef<[u8]> for Ed25519SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn ed25519_secret_as_ref() {
    let inner = [0; ed25519_dalek::SECRET_KEY_LENGTH];
    let secret = Ed25519SecretKey(inner);

    assert_eq!(&inner, secret.as_ref(),);
}

impl From<&ed25519_dalek::Keypair> for Ed25519SecretKey {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Ed25519SecretKey::from(keypair.secret.to_bytes())
    }
}

#[cfg(test)]
#[test]
fn ed25519_secret_from_dalek_keypair() {
    let mut csprng = rand::rngs::OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    let secret = crate::ed25519::secret::Ed25519SecretKey::from(&keypair);

    assert_eq!(keypair.secret.to_bytes(), secret.as_ref(),);
}
