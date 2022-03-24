// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! A stupidly simple QR code renderer, that prints text as QR code to the terminal,
//! and nothing else.
//!
//! # Examples
//! [`example.rs`](./example/example.rs):
//! ```rust
//! qr2term::print_qr("https://rust-lang.org/").unwrap();
//! ```
//!
//! ![qr2term example screenshot](./res/qr2term-example.png)
//!
//! # Based on
//! This library is based on [`qair`](https://code.willemp.be/willem/qair),
//! which didn't provide the renderer as a library on it's own.
//! Credits for the actual renderer go to it's developer.
//!
//! - [https://crates.io/crates/qair](https://crates.io/crates/qair)
//! - [https://code.willemp.be/willem/qair/src/branch/master/src/console_barcode_renderer.rs](https://code.willemp.be/willem/qair/src/branch/master/src/console_barcode_renderer.rs)

pub mod matrix;
pub mod qr;
pub mod render;
pub(crate) mod util;
use qrcode::render::svg;
use qrcode::render::unicode;
pub use qrcode::types::QrError;
use qrcode::{EcLevel, QrCode, Version};

use crate::matrix::Matrix;
use crate::render::Renderer;

/// Quiet zone size in pixels around QR code.
///
/// Should be 4, but using 2 for small terminals:
/// https://qrworld.wordpress.com/2011/08/09/the-quiet-zone/
const QUIET_ZONE_WIDTH: usize = 2;

/// Print the given `data` as QR code in the terminal.
///
/// Returns an error if generating the QR code failed.
///
/// # Examples
///
/// ```rust
/// qrcode_term::qr_print("https://rust-lang.org/").unwrap();
/// ```
///
/// # Panics
///
/// Panics if printing the QR code to the terminal failed.
pub fn qr_print<D: AsRef<[u8]>>(data: D) -> Result<(), QrError> {
    // Generate QR code pixel matrix
    let mut matrix = qr::Qr::from(data)?.to_matrix();
    matrix.surround(QUIET_ZONE_WIDTH, render::QrLight);

    // Render QR code to stdout
    Renderer::default().print_stdout(&matrix);
    Ok(())
}

/// Generate `String` from the given `data` as QR code.
///
/// Returns an error if generating the QR code failed.
///
/// # Examples
///
/// ```rust
/// let qr_string = qrcode_term::qr_string("https://rust-lang.org/").unwrap();
/// print!("{}", qr_string);
/// ```
///
/// # Panics
///
/// Panics if generating the QR code string failed.
pub fn qr_string<D: AsRef<[u8]>>(data: D) -> Result<String, QrError> {
    // Generate QR code pixel matrix
    let mut matrix = qr::Qr::from(data)?.to_matrix();
    matrix.surround(QUIET_ZONE_WIDTH, render::QrLight);

    // Render QR code to a String
    let mut buf = Vec::new();
    Renderer::default()
        .render(&matrix, &mut buf)
        .expect("failed to generate QR code string");
    Ok(String::from_utf8(buf).unwrap())
}

// pub fn print_qr<D: AsRef<[u8]>>(data: D) -> Result<(), QrError> {
//     let code = QrCode::new(data).unwrap();
//     // unicode string qrcode
//     let unicode_qrcode = generate_qr_unicode(code);
//     println!("{}", unicode_qrcode);
//     Ok(())
// }

/// Generate `String` from the given `data` as QR code.
///
/// Returns an error if generating the QR code failed.
///
/// # Examples
///
/// ```rust
/// let u8_arr = qrcode_term::qr_bytes("https://rust-lang.org/").unwrap();
/// print!("{}", u8_arr);
/// ```
///
/// # Panics
///
/// Panics if generating the QR code string failed.
pub fn qr_bytes<D: AsRef<[u8]>>(data: D) -> Result<Vec<u8>, QrError> {
    let code = QrCode::new(data).unwrap();
    // unicode string qrcode
    let unicode_qrcode = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    Ok(Vec::from(unicode_qrcode.as_bytes()))
}

/// Generate `String` from the given `data` as QR code.
///
/// Returns an error if generating the QR code failed.
///
/// # Examples
///
/// ```rust
/// let svg_str = qrcode_term::qr_svg("https://rust-lang.org/").unwrap();
/// print!("{}", svg_str);
/// ```
///
/// # Panics
///
/// Panics if generating the QR code string failed.
pub fn qr_svg<D: AsRef<[u8]>>(data: D) -> Result<String, QrError> {
    let code = QrCode::with_version(data, Version::Normal(5), EcLevel::M).unwrap();
    let svg = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();
    Ok(svg)
}

// pub fn qr_image<D: AsRef<[u8]>>(data: D, path: D) {
//     qrcode::qr_image(data, path);
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_api() {
        // 终端打印二维码
        qr_print("https://github.com/zf1976/pancli").unwrap();

        // 二维码字节数组
        let qrcode_bytes = qr_bytes("https://github.com/zf1976/pancli").unwrap();
        println!("{:?}", qrcode_bytes.as_slice());
    }
}
