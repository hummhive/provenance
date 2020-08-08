pub(crate) mod app;
mod jwt;
mod pubkey;

use std::convert::TryFrom;
use humm_crypto as crypto;

fn main() {
    let app = crate::app::cli::app();
    let matches = app.get_matches();

    // unwrap the jwt and pubkey
    // this is an unwrap because args are required by the clap builder
    let jwt_token = humm_jwt::token::Token::from(matches.value_of(crate::jwt::constant::NAME).unwrap());
    // wrap the raw input in `"` so it can be treated as a json string internally
    let pubkey_portable = crypto::ed25519::public::PubKeyPortable::from(
        format!(
            r#""{}""#,
            matches.value_of(
                crate::pubkey::constant::NAME
            ).unwrap()
        )
    );

    println!("{:?}", jwt_token);
    println!("{:?}", pubkey_portable);

    let pubkey = crypto::ed25519::public::Ed25519PubKey::try_from(&pubkey_portable);

    dbg!(&pubkey);
}
