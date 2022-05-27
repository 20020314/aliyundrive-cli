use base64::{CharacterSet, Config};
use std::ffi::OsString;
use std::os::unix::prelude::OsStringExt;
use std::string::String;

fn main() {
    let str = "";
    let vec = base64::decode_config(str, Config::new(CharacterSet::Standard, false)).unwrap();
    println!("{:?}", OsString::from_vec(vec))
}
