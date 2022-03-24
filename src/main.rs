use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    qrcode_term::qr_print("https://httpbin.org/ip").unwrap();
    qrcode_term::qr_svg("https://httpbin.org/");
    qrcode::qr_image("data", "/tmp/qrcode.png");
    Ok(())
}
