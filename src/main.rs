mod handler;
mod drive;

use clap::{Parser, Subcommand};
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about, arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CLI {
    /// Enable debug mode
    #[clap(short, long)]
    debug: bool,

    #[clap(short, long)]
    refresh_token: Option<String>,

    #[clap(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start the daemon to refresh the token
    Daemon,
    /// Scan the qrcode to login to obtain token or other information
    #[clap(arg_required_else_help = true)]
    #[clap(subcommand)]
    QR(QrCommand),
    /// Sets a custom config file
    #[clap(arg_required_else_help = true)]
    Config {
        /// Print configuration
        #[clap(long)]
        cat: bool,

        /// Read token
        #[clap(long)]
        cat_token: bool,
    },
}

#[derive(Subcommand, Debug)]
enum QrCommand {
    /// Scan QRCode login to get a refresh token
    #[clap(arg_required_else_help = true)]
    Login {
        /// Mobile App QRCode scan code login
        #[clap(long, group = "login")]
        app: bool,
        /// Web QRCode scan code login
        #[clap(long, group = "login")]
        web: bool,
        /// Save the login token to a file
        #[clap(long, short, requires = "login")]
        sava: bool,
    },
    /// Generate QRCode content and query parameters
    Generate,
    /// Query the QRCode login result
    #[clap(arg_required_else_help = true)]
    Query {
        /// Query parameter t
        #[clap(long)]
        t: i64,
        /// Query parameter ck
        #[clap(long)]
        ck: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();
    // enabled debug mode
    init_log(cli.debug);
    // subcommands handlers
    handler::subcommands_handler(&cli.commands).await?;
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
                chrono::Local::now().format(drive::standard::TIME_FORMAT),
                record.args()
            )
        })
        .init();
}
