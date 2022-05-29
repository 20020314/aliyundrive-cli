use clap::{arg, command, Command};
use std::path::Path;

fn main() {
    let matches = Command::new("aliyundrive-cli")
        .about("Alibaba Cloud Disk Terminal CLI Tool")
        .arg(
            arg!(-t --token <TOKEN>)
                .default_missing_value("aaaa")
                .required(false),
        )
        .get_matches();

    println!("two: {:?}", matches.value_of("two").expect("required"));
    println!("one: {:?}", matches.value_of("one").expect("required"));
}
