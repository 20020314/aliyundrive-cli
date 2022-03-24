use qrcode::QrCode;

fn main() {
    // print qrcode
    qrcode_term::qr_print("https://").unwrap();

    // print qrcode unicode string
    let string = qrcode_term::qr_string("https://").unwrap();
    println!("{}", string);

    // print qrcode u8 arr
    let u8_arr = qrcode_term::qr_bytes("https://rust-lang.org/").unwrap();
    println!("{:?}", u8_arr);

    // print qrcode svg String
    let svg_str = qrcode_term::qr_svg("https://rust-lang.org/").unwrap();
    println!("{:?}", svg_str);

    // image qrcode
    qrcode::qr_image("https://rust-lang.org/", "/tmp/qrcode.png");
}
