mod conf;
mod handler;

use clap::{arg, Arg, Command};
use std::io::Write;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "<gngpp verticle@foxmail.com>", version, about = "Alibaba Cloud Disk Terminal CLI Tool", long_about = None, arg_required_else_help = true)]
pub struct CLI {
    /// Sets a custom config file
    #[clap(short = 'c', long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Enable debug mode
    #[clap(short = 'd', long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan the qrcode to log in to obtain token or other information
    #[clap(arg_required_else_help = true)]
    QRCODE {
        /// Mobile QR Code scan code login
        #[clap(long)]
        mobile_token: bool,

        /// Web QR Code scan code login
        #[clap(long)]
        web_token: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();
    if cli.debug {
        std::env::set_var("RUST_LOG", "DEBUG");
        log::debug!("debug log")
    }
    init_log();
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    Ok(())
}

fn init_log() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}: {}",
                record.level(),
                //Format like you want to: <-----------------
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.args()
            )
        })
        .init();
}
