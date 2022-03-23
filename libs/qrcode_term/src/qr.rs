
use crate::{render::unicode, types::QrError, QrCode};


pub fn print_qr<D: AsRef<[u8]>>(data: D) -> Result<(), QrError> {
    let code = QrCode::new(data).unwrap();
    // unicode string qrcode
    let unicode_qrcode = generate_qr_unicode(code);
    println!("{}", unicode_qrcode);
    Ok(())
}

pub fn generate_qr_bytes<D: AsRef<[u8]>>(data: D) -> Result<Vec<u8>, QrError> {
    let code = QrCode::new(data).unwrap();
    // unicode string qrcode
    let unicode_qrcode = generate_qr_unicode(code);
    Ok(Vec::from(unicode_qrcode.as_bytes()))
}



fn generate_qr_unicode(code: QrCode) -> String {
    code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_api() {
        // 终端打印二维码
        print_qr("https://github.com/zf1976/pancli").unwrap();

        // 二维码字节数组
        let qrcode_bytes = generate_qr_bytes("https://github.com/zf1976/pancli").unwrap();
        println!("{:?}", qrcode_bytes.as_slice());

    }
}
