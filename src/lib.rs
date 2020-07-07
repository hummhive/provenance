pub mod env;
pub mod json;
pub mod error;
pub mod crypto;
pub mod content;
pub mod version;
pub mod roughtime;
pub mod jwt;
pub mod device;

#[derive(serde::Serialize)]
struct Provenance {
    pub version: version::Version,
    pub content_hash: content::Hash,
    pub roughtime: roughtime::Roughtime,
    pub jwt: jwt::Jwt,
    pub device_signature: device::signature::Signature,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[test]
    fn make_provenance() {
        let mut csprng = rand::rngs::OsRng{};
        let device_keypair: ed25519_dalek::Keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let idp_keypair: ed25519_dalek::Keypair = ed25519_dalek::Keypair::generate(&mut csprng);
        let content = content::Content::from(b"foo".to_vec());
        let version = version::Version::Zero;

        // for simplicity's sake the device and idp can share the same ecosystem
        // on the happy path this is going to be true 99% of the time as we'd expect everyone to
        // just use the ecosystem.json that cloudflare provides exactly like we do
        let ecosystem = roughtime::ecosystem::Ecosystem::try_from(roughtime::ecosystem::env::EcosystemJsonFilePath).unwrap();

        // first part of what happens on the device
        // the device will lookup its own keypair and opinion on valid timeservers to request a
        // JWT from the IDP in the next phase
        let device_sends_pub_keys_on_wire = || -> (device::keys::PubKeyPortable, roughtime::ecosystem::server::public_key::KeysPortable) {
            // device has access to its keypair
            ((&device_keypair).try_into().unwrap(), (&ecosystem).try_into().unwrap())
        };

        // this bit happens on the IDP
        // the device would send the IDP (at least) its own pub key and time pub keys list over the
        // network as strings
        // the IDP decides if it trusts both the device and time pub keys, and if so it sends back
        // an expiring JWT token string that includes the device pub and sha512 of the time keys
        // this would look something like how github allows users to upload pub keys to access git
        // repos via. ssh
        // the input and output is all String based due to this being designed to work over the
        // network but the internals are all strongly typed
        // this also means additional failure points and checks in each direction to go in and out
        // of strings
        let idp_sends_back_signed_jwt = |device_pub_portable: device::keys::PubKeyPortable, time_pubs_portable: roughtime::ecosystem::server::public_key::KeysPortable| -> Option<jwt::token::Token> {
            let device_pub: device::keys::PubKey = (&device_pub_portable).try_into().unwrap();
            let time_pubs: roughtime::ecosystem::server::public_key::Keys = (&time_pubs_portable).try_into().unwrap();

            // the IDP needs to decide whether all the pub keys it is being asked to include in the
            // JWT are known and trustworthy
            // if not, the device doesn't receive a JWT
            // @todo maybe is_trustworthy() is simply internal to a try_from()?
            Some((&jwt::JwtInput {
                // the IDP has access to its own keypair
                idp_keypair: (&idp_keypair).into(),
                // and trusts the device pub
                device_pub_key: device_pub,
                time_keys_hash: (&time_pubs).into(),
            }).try_into().unwrap())
        };

        let device_signs_content_and_jwt = |
            version: &version::Version,
            content: &content::Content,
            jwt_token: &jwt::token::Token,
        | -> Option<device::signature::Signature> {
            // device includes the version byte so that the algorithm can be replayed reliably
            // later in the event of new provenance proof versions
            let version_int: version::VersionInt = version.into();

            // the device needs to sign the content it is establishing provenance over
            // we want to be able to efficiently verify the signature so we hash the content first
            let content_hash: content::Hash = content.into();

            // device only need to sign the jwt's signature because jwts are already self-validating
            // much like the provenance is
            // device would check all the claims in the jwt match everything that will be included
            // in the final provenance json
            //
            // device signing the jwt signature:
            // - asserts the jwt is valid according to standard jwt verification
            // - accepts the idp providing an identity
            // - asserts the identity provided by the idp is correct
            // - asserts the claims such as time pubs hash and device aud and expiry are correct
            //
            // device must decide for itself whether it believes the expiry time etc. are valid
            // because if not it may sign itself against a bung proof that nobody will accept later
            let jwt_signature: jwt::token::Signature = jwt_token.try_into().unwrap();

            Some((&device::signature::SignatureInput {
                device_keypair: (&device_keypair).into(),
                version_int,
                content_hash,
                jwt_signature,
            }).try_into().unwrap())
        };

        let (device_pub_key_portable, time_pub_keys_portable) = device_sends_pub_keys_on_wire();

        let idp_signed_jwt_portable = idp_sends_back_signed_jwt(device_pub_key_portable, time_pub_keys_portable).unwrap();

        let device_signature = device_signs_content_and_jwt(
            &version,
            &content,
            &idp_signed_jwt_portable,
        ).unwrap();

        println!("zzz {:?}", serde_json::to_string(&device_signature).unwrap());

        // let content = content::Content::from(b"foo".to_vec());
        //
        //
        // let jwt_input = jwt::JwtInput {
        //     idp_keypair: (&idp_keypair).into(),
        //     device_pub_key: (&device_keypair).into(),
        //     time_keys_hash: (&ecosystem).into(),
        // };
        //

        let roughtime: roughtime::Roughtime = roughtime::RoughtimeInput {
            ecosystem: ecosystem.clone(),
            device_signature: device_signature.clone(),
        }.try_into().unwrap();

        let provenance = Provenance {
            version,
            content_hash: (&content).into(),
            roughtime,
            jwt: jwt::Jwt {
                idp_pub_key: (&idp_keypair).into(),
                token: idp_signed_jwt_portable,
            },
            device_signature,
        };

        let output_path = std::env::var("HUMM_PROVENANCE_OUTPUT_PATH").unwrap();

        let json_output = serde_json::to_string_pretty(&provenance).unwrap();
        println!("{}", json_output);
        std::fs::write(output_path, json_output).unwrap();
    }

}
