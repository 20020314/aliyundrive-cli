mod conf;
mod handler;

use std::io::Write;
use clap::{Parser, Subcommand};
use crate::conf::Context;

#[derive(Parser, Debug)]
#[clap(author = "<gngpp verticle@foxmail.com>", version, about = "Alibaba Cloud Disk Terminal CLI Tool", long_about = None, arg_required_else_help = true)]
pub struct CLI {

    /// Enable debug mode
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan the qrcode to log in to obtain token or other information
    #[clap(arg_required_else_help = true)]
    QRCODE {
        /// Mobile QRCode scan code login
        #[clap(long, short, group = "token")]
        mobile_token: bool,
        /// Web QRCode scan code login
        #[clap(long, short, group = "token")]
        web_token: bool,
        /// Save the login token to a file
        #[clap(long, short, requires = "token")]
        sava: bool
    },
    /// Sets a custom config file
    #[clap(arg_required_else_help = true)]
    CONFIG {

    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();
    // enabled debug mode
    init_log(cli.debug);
    // subcommands
    match &cli.commands {
        Some(Commands::QRCODE {
            web_token,
            mobile_token,
            sava
        }) => {
            // qrcode scan
            if *web_token || *mobile_token {
                handler::qrcode_token_handler(*web_token, *mobile_token, *sava).await?
            }
        }
        Some(Commands::CONFIG {}) => {
            Context::init()?;
        }
        None => {}
    }

    Ok(())
}

fn init_log(debug: bool) {
    if debug {
        std::env::set_var("RUST_LOG", "DEBUG");
    } else {
        std::env::set_var("RUST_LOG", "INFO");
    }
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
