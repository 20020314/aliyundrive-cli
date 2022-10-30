use crate::{Commands, QrCommand};

pub(crate) async fn subcommands_handler(subcommands: &Option<Commands>) -> anyhow::Result<()> {
    match &subcommands {
        Some(Commands::QR(QrCommand::Login { app, web, sava })) => {
            // qrcode scan
            if *web || *app {
                let credentials =
                    crate::drive::login::QrCodeHandler::qrcode_scan_handler(*web, *app).await?;
                println!("{}", serde_json::to_string_pretty(&credentials)?);
                // Sava the authorization token to config file
                if *sava {
                    crate::drive::conf::AppConf::write(credentials).await?;
                }
            }
        }
        Some(Commands::QR(QrCommand::Generate)) => {
            crate::drive::login::QrCodeHandler::print_qrcode_content_std().await?;
        }
        Some(Commands::QR(QrCommand::Query { t, ck })) => {
            let credentials =
                crate::drive::login::QrCodeHandler::query_qrcode_app_login_result(t.clone(), ck.clone())
                    .await?;
            crate::drive::conf::AppConf::write(credentials).await?;
        }
        Some(Commands::Config { cat, cat_token }) => {
            if *cat {
                crate::drive::conf::AppConf::print_std().await?;
            }

            if *cat_token {
                crate::drive::conf::AppConf::print_token().await?;
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
