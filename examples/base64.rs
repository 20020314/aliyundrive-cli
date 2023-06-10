use std::string::String;

fn main() {
    // let mut saa = mem::ManuallyDrop::new(saa);
    //
    // let ptr = saa.as_mut_ptr();
    // let len = saa.len();
    // let capacity = saa.capacity();
    //
    // let saa = unsafe { String::from_raw_parts(ptr, len, capacity) };

    // assert_eq!(String::from("hello"), saa);
    let str = "";
    let vec = base64::decode(str).unwrap();
    let string = vec.iter().map(|&c| c as char).collect::<String>();
    println!("{}", string);
    let string1 = String::from_utf8_lossy(vec.as_slice()).to_string();
    println!("{}", string1);
}
