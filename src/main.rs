use clap::arg;
use conf::rw::RW;
use conf::{Config, Context};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    clap::Command::new("aliyundrive-cli")
        .about("Alibaba Cloud Disk Terminal CLI Tool")
        .arg(arg!(-t --token <TOKEN>).help("").required(false))
        .get_matches();
    Context::write(conf::Config::default())?;
    let config = Context::read().unwrap();
    println!("{:?}", config);
    Ok(())
}
