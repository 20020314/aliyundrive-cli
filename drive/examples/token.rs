use drive::scan::model::auth::{AuthorizationCode, Token};
use drive::scan::model::query::QueryQrCodeCkForm;
use drive::scan::model::{AuthorizationToken, Ok};
use drive::scan::qr::QrCodeScanner;
use std::io::Write;
use std::{thread, time};

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}: {}",
                record.level(),
                //Format like you want to: <-----------------
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.args()
            )
        })
        .init();
    let scan = QrCodeScanner::new().unwrap();
    let generator_result = scan.generator().unwrap();
    qrcode::qr_print(generator_result.get_qrcode_content()).expect("print qrcode error.");
    let ck_form = QueryQrCodeCkForm::from(generator_result);
    for _i in 0..10 {
        thread::sleep(time::Duration::from_secs(2));
        let query_result = scan.query(&ck_form).unwrap();
        if query_result.ok() {
            // query_result.is_new() 表示未扫码状态
            if query_result.is_new() {
                println!("new");
                // 做点什么..
                continue;
            }
            // query_result.is_expired() 表示扫码成功，但未点击确认登陆
            if query_result.is_expired() {
                // 做点什么..
                println!("expired");
                continue;
            }
            // 移动端APP扫码成功并确认登陆
            if query_result.is_confirmed() {
                let mobile_login_result = query_result.get_mobile_login_result().unwrap();
                println!("mobile_login_result: {:#?}", mobile_login_result);

                let access_token = mobile_login_result.access_token().unwrap();
                println!("mobile_login_result-access_token: {}\n", access_token);

                let refresh_token = mobile_login_result.refresh_token().unwrap();
                println!("mobile_login_result-refresh_token: {}\n", refresh_token);

                let goto_result = scan.token_login(Token::from(&access_token)).unwrap();
                println!("goto result: {:#?}\n", goto_result);

                let authorization_code = goto_result.extract_authorization_code().unwrap();
                println!("authorization_code: {}", authorization_code);

                let web_login_result = scan
                    .get_token(AuthorizationCode::from(&goto_result))
                    .unwrap();
                println!("web_login_result: {:#?}", web_login_result);

                let access_token = web_login_result.access_token().unwrap();
                println!("web_login_result-access_token: {}\n", access_token);

                let refresh_token = web_login_result.refresh_token().unwrap();
                println!("web_login_result-refresh_token: {}\n", refresh_token);
            }
        }
    }
}
