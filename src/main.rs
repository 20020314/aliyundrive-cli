mod handler;

use anyhow::Context;
use clap::{Parser, Subcommand};
use drive::scan::model::AuthorizationToken;
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about, arg_required_else_help = true)]
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
    },
}

#[derive(Subcommand, Debug)]
enum QrCommand {
    /// Scan QRCode login to get a refresh token
    #[clap(arg_required_else_help = true)]
    Scan {
        /// Mobile App QRCode scan code login
        #[clap(long, short, group = "token")]
        app: bool,
        /// Web QRCode scan code login
        #[clap(long, short, group = "token")]
        web: bool,
        /// Save the login token to a file
        #[clap(long, short, requires = "token")]
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
    // subcommands
    subcommands_handler(&cli.commands).await?;
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

async fn subcommands_handler(subcommands: &Option<Commands>) -> anyhow::Result<()> {
    match &subcommands {
        Some(Commands::QR(QrCommand::Scan { app, web, sava })) => {
            // qrcode scan
            if *web || *app {
                let authorization = handler::qrcode_scan_handler(*web, *app).await?;
                println!("{}", serde_json::to_string_pretty(&authorization)?);
                // Sava the authorization token to config file
                if *sava {
                    // saa::Context::init()?;
                    // let authorization_token = saa::Authorization::new(None, Some(refresh_token));
                    // saa::Context::write_token(*mobile_token, authorization_token)
                    //     .context("Failed to save configuration!")?
                }
            }
        }
        Some(Commands::QR(QrCommand::Generate)) => {
            let scan = drive::scan::qr::QrCodeScanner::new().await?;
            let result = scan.qrcode_generator().await?;
            let data = result.get_content_data().context("failed to get QRCode")?;
            println!("{}", serde_json::to_string_pretty(&data)?);
        }
        Some(Commands::QR(QrCommand::Query { t, ck })) => {
            let scan = drive::scan::qr::QrCodeScanner::new().await?;
            let form = drive::scan::model::query::QueryQrCodeCkForm::new((*t, ck.to_string()));
            let query_result = scan.do_get_query_response(&form).await?;
            if query_result.is_confirmed() {
                let refresh_token = query_result
                    .get_app_login_result()
                    .context("failed to get mobile login result")?
                    .refresh_token()
                    .context("failed to get refresh token")?;
                println!("{}", refresh_token)
            }
        }
        Some(Commands::Config { cat }) => {}
        Some(Commands::Daemon) => loop {
            log::info!("daemon running..");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
        },
        None => {}
    }

    Ok(())
}
