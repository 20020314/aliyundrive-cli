mod handler;

use clap::{arg, Arg, Command};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let matches = Command::new("aliyundrive-cli")
        .arg_required_else_help(true)
        .author("gngpp <verticle@foxmail.com>")
        .about("Alibaba Cloud Disk Terminal CLI Tool")
        .version("1.0")
        .arg(arg!([NAME] "Optional name to operate on"))
        .arg(
            Arg::new("OPTIONS")
                .short('c')
                .long("conf")
                .default_missing_value("")
                .help("Configure Authorization Token")
                .required(false),
        )
        .arg(
            Arg::new("LOGIN")
                .short('L')
                .long("login")
                .default_missing_value("")
                .help("Mobile QR Code scan code login")
                .required(false),
        )
        .get_matches();
    if matches.is_present("LOGIN") {
        println!("{:?}", "login")
    }
    Ok(())
}
