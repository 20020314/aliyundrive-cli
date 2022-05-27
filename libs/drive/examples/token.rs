use drive::login::qr::LoginQrCodeScanner;
use drive::login::{QrCodeScanner, QrCodeScannerState};
use drive::models::query::QueryQrCodeCkForm;
use drive::models::Ok;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::{thread, time};

fn main() {
    let scanner = LoginQrCodeScanner::new();
    let generator_result = scanner.get_generator_result().unwrap();
    let ck_form = QueryQrCodeCkForm::from(generator_result);
    println!("{:#?}", &ck_form);
    qrcode::qr_print(ck_form.get_content()).expect("print qrcode error.");

    loop {
        let query_result = scanner.get_query_result(&ck_form).unwrap();
        if query_result.ok() {
            if query_result.is_confirmed() {
                let result = query_result.get_biz_ext();
                if let Some(result) = result {
                    let vec = base64::decode(result).unwrap();
                    println!("{:?}", OsString::from_vec(vec));
                    break;
                }
            }
        }
        thread::sleep(time::Duration::from_secs(2));
    }
}
