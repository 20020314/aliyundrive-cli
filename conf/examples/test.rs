use std::{thread, time};
use conf::Context;
use conf::rw::RW;
use drive::models::auth::{AuthorizationCode, Token};
use drive::models::{AuthorizationToken, Ok};
use drive::models::query::QueryQrCodeCkForm;
use drive::scan::qr::LoginQrCodeScanner;
use drive::scan::{QrCodeScanner, QrCodeScannerState};

fn main() {
    let scanner = LoginQrCodeScanner::new();
    let generator_result = scanner.get_generator_result().unwrap();
    let ck_form = QueryQrCodeCkForm::from(&generator_result);
    qrcode::qr_print(ck_form.get_content()).expect("print qrcode error.");
    println!("{:#?}", &ck_form);
    loop {
        let query_result = scanner.get_query_result(&ck_form).unwrap();
        if query_result.ok() {
            if query_result.is_expired() {
                break;
            }
            if query_result.is_confirmed() {
                let mobile_login_result = query_result.get_mobile_login_result().unwrap();
                let mobile_access_token = mobile_login_result.access_token().unwrap();
                let mobile_refresh_token = mobile_login_result.refresh_token().unwrap();
                let goto_result = scanner.token_login(Token::from(&mobile_access_token)).unwrap();
                let web_login_result = scanner
                    .get_token(AuthorizationCode::from(&goto_result))
                    .unwrap();
                let web_access_token = web_login_result.access_token().unwrap();
                let web_refresh_token = web_login_result.refresh_token().unwrap();
                let p1 = conf::AuthorizationToken::new(mobile_access_token, mobile_refresh_token);
                let p2 = conf::AuthorizationToken::new(web_access_token, web_refresh_token);
                let config = conf::Config::new(Some(p1), Some(p2));
                Context::write(config).unwrap();
                let c = Context::read().unwrap();
                println!("{:#?}", c);
            }
        }
        thread::sleep(time::Duration::from_secs(2));
    }
}