use drive::models::query::QueryQrCodeCkForm;
use drive::models::Ok;
use drive::token::qr::AliyunQrCodeScanner;
use drive::token::QrCodeScanner;
use std::{thread, time};

fn main() {
    let scanner = AliyunQrCodeScanner::new();
    let generator_result = scanner.get_generator_result().unwrap();
    let ck_form = QueryQrCodeCkForm::from(generator_result);
    println!("{:#?}", &ck_form);
    qrcode::qr_print(ck_form.get_content()).expect("print qrcode error.");

    loop {
        let query_result = scanner.get_query_result(&ck_form).unwrap();
        if query_result.ok() {
            if let Some(content) = query_result.content {
                if let Some(data) = content.data {
                    let result = data.biz_ext.unwrap_or_default();
                    if !result.is_empty() {
                        let vec = base64::decode(result).unwrap();
                        println!("{:?}", String::from_utf8(vec));
                        break;
                    }
                }
            }
        }
        thread::sleep(time::Duration::from_secs(2));
    }
}
