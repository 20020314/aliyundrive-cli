use base64::{CharacterSet, Config};
use std::os::unix::prelude::OsStringExt;
use std::string::String;

fn main() {
    // let mut s = mem::ManuallyDrop::new(s);
    //
    // let ptr = s.as_mut_ptr();
    // let len = s.len();
    // let capacity = s.capacity();
    //
    // let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    // assert_eq!(String::from("hello"), s);
    let str = "";
    let vec = base64::decode(str).unwrap();
    let string = vec.iter().map(|&c| c as char).collect::<String>();
    println!("{}", string);
    let string1 = String::from_utf8_lossy(vec.as_slice()).to_string();
    println!("{}", string1);
}
