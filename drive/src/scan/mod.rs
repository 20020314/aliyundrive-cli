pub mod qr;

use crate::models::{auth, gen, query, suc};

#[allow(dead_code)]
pub enum State {
    CONFIRMED,
    EXPIRED,
    NEW,
}

impl State {
    pub fn eq(&self, str: &String) -> bool {
        let value = self.to_string();
        return str.eq(&value);
    }
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
    fn get_generator_result(&self) -> crate::ScanResult<gen::GeneratorQrCodeResult>;

    // query qrcode result
    fn get_query_result(
        &self,
        from: &query::QueryQrCodeCkForm,
    ) -> crate::ScanResult<query::QueryQrCodeResult>;

    // scan scan result（include authorization code）
    fn token_login(&self, token: auth::Token) -> crate::ScanResult<suc::GotoResult>;

    // get web side scan
    fn get_token(&self, auth: auth::AuthorizationCode) -> crate::ScanResult<suc::WebLoginResult>;
}

pub trait QrCodeScannerState {
    fn is_new(&self) -> bool;

    fn is_expired(&self) -> bool;

    fn is_confirmed(&self) -> bool;
}
