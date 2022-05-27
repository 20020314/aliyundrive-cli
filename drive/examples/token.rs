use drive::login::qr::LoginQrCodeScanner;
use drive::login::{QrCodeScanner, QrCodeScannerState};
use drive::models::query::QueryQrCodeCkForm;
use drive::models::{suc, Ok};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::{thread, time};
use drive::models::auth::Token;

fn main() {
    let scanner = LoginQrCodeScanner::new();
    let generator_result = scanner.get_generator_result().unwrap();
    let ck_form = QueryQrCodeCkForm::from(&generator_result);
    println!("{:#?}", &ck_form);
    qrcode::qr_print(ck_form.get_content()).expect("print qrcode error.");

    loop {
        let query_result = scanner.get_query_result(&ck_form).unwrap();
        if query_result.ok() {
            if query_result.is_expired() {
                break;
            }
            if query_result.is_confirmed() {
                let mobile_login_result = query_result.get_mobile_login_result().unwrap();
                let access_token = mobile_login_result.get_access_token().unwrap();
                println!("mobile_login_result-access_token: {}\n", access_token);
                let refresh_token = mobile_login_result.get_refresh_token().unwrap();
                println!("mobile_login_result-refresh_token: {}\n", refresh_token);
                let goto_result = scanner.token_login(Token::from(&access_token)).unwrap();
                let authorization_code = goto_result.extract_authorization_code().unwrap();
                println!("authorization_code: {}", authorization_code);
                println!("goto result: {:?}\n", goto_result);
            }
        }
        thread::sleep(time::Duration::from_secs(2));
    }
}
