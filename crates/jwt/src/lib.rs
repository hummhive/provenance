pub mod error;
pub mod signature;
pub mod token;

use humm_provenance_idp as idp;

pub struct Jwt {
    pub idp_pub_key: idp::key::PubKey,
    pub token: token::Token,
}
