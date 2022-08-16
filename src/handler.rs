use crate::{Commands, QrCommand};
use anyhow::Context;
use drive::conf::AppConf;
use drive::scan::model::{auth, AuthorizationToken, Ok};

pub(crate) async fn subcommands_handler(subcommands: &Option<Commands>) -> anyhow::Result<()> {
    match &subcommands {
        Some(Commands::QR(QrCommand::Scan { app, web, sava })) => {
            // qrcode scan
            if *web || *app {
                let credentials = drive::scan::ScanHandler::qrcode_scan_handler(*web, *app).await?;
                println!("{}", serde_json::to_string_pretty(&credentials)?);
                // Sava the authorization token to config file
                if *sava {
                    AppConf::write(credentials).await?;
                }
            }
        }
        Some(Commands::QR(QrCommand::Generate)) => {
            drive::scan::ScanHandler::print_qrcode_content_std().await?;
        }
        Some(Commands::QR(QrCommand::Query { t, ck })) => {
            let credentials = drive::scan::ScanHandler::query_qrcode_app_login_result(t.clone(), ck.clone()).await?;
            AppConf::write(credentials).await?;
        }
        Some(Commands::Config { cat }) => {
            if *cat {
                AppConf::print_std().await?;
            }
        }
        Some(Commands::Daemon) => loop {
            log::info!("daemon running..");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
        },
        None => {}
    }

    Ok(())
}
