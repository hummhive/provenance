impl From<&Vec<u8>> for Sha512Hash {
    fn from(bytes: &Vec<u8>) -> Self {
        let mut hash = [0; SHA512_OUTPUT_LEN];
        hash.copy_from_slice(
            &ring::digest::digest(&ring::digest::SHA512, &bytes)
                .as_ref()
                .to_owned(),
        );
        Self(hash)
    }
}
