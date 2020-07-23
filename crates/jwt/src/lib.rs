pub mod error;
pub mod idp;
pub mod signature;
pub mod token;

#[derive(serde::Serialize)]
pub struct Jwt {
    pub idp_pub_key: idp::PubKey,
    pub token: token::Token,
}
