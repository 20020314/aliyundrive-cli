pub mod qr;

use crate::models::{gen, query, suc};
use serde::Serialize;

#[allow(dead_code)]
pub enum State {
    CONFIRMED,
    EXPIRED,
    NEW,
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::NEW => "NEW".to_string(),
            State::EXPIRED => "EXPIRED".to_string(),
            State::CONFIRMED => "CONFIRMED".to_string(),
        }
    }
}

pub trait QrCodeScanner {
    // get qrcode generator result contend.
    fn get_generator_result(&self) -> crate::Result<gen::GeneratorQrCodeResult>;

    // query qrcode result
    fn get_query_result(
        &self,
        from: &query::QueryQrCodeCkForm,
    ) -> crate::Result<query::QueryQrCodeResult>;

    //  get session id
    fn get_session_id(&self) -> crate::Result<String>;

    // token login result（include authorization code）
    fn token_login(&self) -> crate::Result<suc::TokenLoginResult>;

    // get web side token
    fn get_token(&self);
}
