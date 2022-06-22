extern crate core;

use core::time;
use drive::scan::model::auth::MobileAccessToken;
use drive::scan::model::query::QueryQrCodeCkForm;
use drive::scan::model::{AuthorizationToken, Ok};
use drive::scan::qr::QrCodeScanner;

#[tokio::main]
async fn main() {
    let mut scan = QrCodeScanner::new().await.unwrap();
    let generator_response = scan.qrcode_generator().await.unwrap();
    qrcode::qr_print(generator_response.get_qrcode_content()).expect("print qrcode error.");
    let form = QueryQrCodeCkForm::from(generator_response);
    for _i in 0..10 {
        tokio::time::sleep(time::Duration::from_secs(3)).await;
        let query_response = scan.do_get_mobile_response(&form).await.unwrap();
        if query_response.ok() {
            // query_result.is_new() 表示未扫码状态
            if query_response.is_new() {
                println!("new");
                // 做点什么..
                continue;
            }
            // query_result.is_expired() 表示扫码成功，但未点击确认登陆
            if query_response.is_expired() {
                // 做点什么..
                println!("expired");
                continue;
            }
            // 移动端APP扫码成功并确认登陆
            if query_response.is_confirmed() {
                let mobile_login_response = query_response.get_mobile_login_result().unwrap();
                println!("mobile_login_result: {:#?}", mobile_login_response);

                let access_token = mobile_login_response.access_token().unwrap();
                println!("mobile_login_result-access_token: {}\n", access_token);

                let refresh_token = mobile_login_response.refresh_token().unwrap();
                println!("mobile_login_result-refresh_token: {}\n", refresh_token);

                let web_login_response = scan
                    .do_get_web_token_response(MobileAccessToken::from(&access_token))
                    .await
                    .unwrap();
                println!("web_login_result: {:#?}", web_login_response);

                let access_token = web_login_response.access_token().unwrap();
                println!("web_login_result-access_token: {}\n", access_token);

                let refresh_token = web_login_response.refresh_token().unwrap();
                println!("web_login_result-refresh_token: {}\n", refresh_token);
                break;
            }
        }
    }
}
