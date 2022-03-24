use std::collections::HashMap;
use qrcode_term::qr;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    qr::print_qr("https://httpbin.org/ip").unwrap();
    Ok(())
}