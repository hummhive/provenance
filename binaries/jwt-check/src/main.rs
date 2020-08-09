pub(crate) mod app;
mod jwt;
mod pubkey;

use std::convert::TryFrom;
use humm_crypto as crypto;

fn main() {
    let app = crate::app::cli::app();
    let matches = app.get_matches();

    let jwt_string = match matches.value_of(crate::jwt::constant::NAME) {
        Some(v) => v,
        None => {
            eprintln!("Error getting jwt from args");
            std::process::exit(exitcode::DATAERR);
        },
    };
    // wrap the raw input in `"` so it can be treated as a json string internally
    let pubkey_portable = crypto::ed25519::public::PubKeyPortable::from(
        format!(
            r#""{}""#,
            match matches.value_of(
                crate::pubkey::constant::NAME
            ) {
                Some(v) => v,
                None => {
                    eprintln!("Error getting pubkey from args");
                    std::process::exit(exitcode::DATAERR);
                },
            }
        )
    );

    let pubkey = crypto::ed25519::public::Ed25519PubKey::try_from(&pubkey_portable).unwrap();

    match humm_jwt::token::Token::try_from((pubkey, jwt_string)) {
        Ok(_) => std::process::exit(exitcode::OK),
        Err(e) => {
            eprintln!("Error validating jwt token: {}", e);
            std::process::exit(exitcode::DATAERR);
        },
    };

}
