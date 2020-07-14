use crate::ed25519::public::Ed25519PubKey;

#[derive(Clone, Copy)]
pub struct Ed25519Keypair([u8; ed25519_dalek::KEYPAIR_LENGTH]);

impl std::convert::TryFrom<&Ed25519Keypair> for Ed25519PubKey {
    type Error = crate::error::CryptoError;
    fn try_from(keypair: &Ed25519Keypair) -> Result<Self, Self::Error> {
        Ok(Self::from(&ed25519_dalek::Keypair::try_from(keypair)?))
    }
}

#[cfg(test)]
#[test]
fn ed25519_keypair_to_public() {
    use std::convert::TryFrom;
    let mut csprng = rand::rngs::OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);

    let public = Ed25519PubKey::from(&keypair);
    let crypto_keypair = Ed25519Keypair::from(&keypair);

    assert_eq!(public, Ed25519PubKey::try_from(&crypto_keypair).unwrap(),);
}

impl std::convert::AsRef<[u8]> for Ed25519Keypair {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
#[test]
fn ed25519_keypair_as_ref() {
    let inner = [0; ed25519_dalek::KEYPAIR_LENGTH];
    let keypair = Ed25519Keypair(inner);

    assert_eq!(&inner.to_vec(), &keypair.as_ref().to_vec());
}

impl From<&ed25519_dalek::Keypair> for Ed25519Keypair {
    fn from(keypair: &ed25519_dalek::Keypair) -> Self {
        Self(keypair.to_bytes())
    }
}

impl std::convert::TryFrom<&Ed25519Keypair> for ed25519_dalek::Keypair {
    type Error = crate::error::CryptoError;
    fn try_from(keypair: &Ed25519Keypair) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(&keypair.0)?)
    }
}

#[cfg(test)]
#[test]
fn ed25519_keypair_from_dalek_keypair() {
    use std::convert::TryFrom;

    let mut csprng = rand::rngs::OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    let crypto_keypair = crate::ed25519::keypair::Ed25519Keypair::from(&keypair);

    assert_eq!(
        keypair.to_bytes().to_vec(),
        crypto_keypair.as_ref().to_vec()
    );

    assert_eq!(
        &keypair.to_bytes().to_vec(),
        &ed25519_dalek::Keypair::try_from(&crypto_keypair)
            .unwrap()
            .to_bytes()
            .to_vec(),
    );
}
