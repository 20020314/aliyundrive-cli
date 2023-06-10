use error::{DriveError, QrCodeScannerError};
use standard::{REQUEST_CONNECT_TIMEOUT, REQUEST_POOL_IDLE_TIMEOUT, REQUEST_TIMEOUT, UA};

pub mod login;
pub mod conf;
pub mod standard;
pub mod error;
pub mod time;

pub type ScanResult<T, E = QrCodeScannerError> = anyhow::Result<T, E>;
pub type DriveResult<T, E = DriveError> = anyhow::Result<T, E>;

pub struct Drive {
    client: reqwest::Client,
    credentials: conf::Credentials,
}

impl Drive {
    pub async fn new() -> DriveResult<Self> {
        let client = reqwest::Client::builder()
            .user_agent(UA)
            .pool_idle_timeout(REQUEST_POOL_IDLE_TIMEOUT)
            .connect_timeout(REQUEST_CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()?;
        Ok(Self {
            client,
            credentials: Default::default(),
        })
    }
}
