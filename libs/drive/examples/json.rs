use drive::qr::{QrCodeScanner, State};

fn main() {
    let resp = QrCodeScanner::get_generator_qrcode_content_result();
    println!("{:#?}", resp);
    // let m = QrCodeScanner::get_query_qrcode_result(&from).unwrap();
    // println!("{:#?}", m);
    println!("{:#?}", State::NEW.to_string());
}
