use drive::models::auth::{AuthorizationCode, Token};
use drive::models::query::QueryQrCodeCkForm;
use drive::models::{Ok, AuthenticationToken};
use drive::scan::qr::LoginQrCodeScanner;
use drive::scan::{QrCodeScanner, QrCodeScannerState};
use std::{thread, time};

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
                println!("mobile_login_result: {:#?}", mobile_login_result);

                let access_token = mobile_login_result.access_token().unwrap();
                println!("mobile_login_result-access_token: {}\n", access_token);

                let refresh_token = mobile_login_result.refresh_token().unwrap();
                println!("mobile_login_result-refresh_token: {}\n", refresh_token);

                let goto_result = scanner.token_login(Token::from(&access_token)).unwrap();
                println!("goto result: {:#?}\n", goto_result);

                let authorization_code = goto_result.extract_authorization_code().unwrap();
                println!("authorization_code: {}", authorization_code);

                let web_login_result = scanner.get_token(AuthorizationCode::from(&goto_result)).unwrap();
                println!("web_login_result: {:#?}", web_login_result);

                let access_token = web_login_result.access_token().unwrap();
                println!("web_login_result-access_token: {}\n", access_token);

                let refresh_token = web_login_result.refresh_token().unwrap();
                println!("web_login_result-refresh_token: {}\n", refresh_token);
            }
        }
        thread::sleep(time::Duration::from_secs(2));
    }
}
