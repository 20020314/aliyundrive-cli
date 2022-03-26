use drive::qr::{QrCodeScanner, State};

fn main() {
    let resp = QrCodeScanner::get_generator_qrcode_content_result();
    println!("{:#?}", resp.unwrap());
    println!("{:#?}", State::NEW.to_string());
}
