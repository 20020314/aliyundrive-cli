use crate::error::{DriveError, QrCodeScannerError};
use crate::r#const::{REQUEST_CONNECT_TIMEOUT, REQUEST_POOL_IDLE_TIMEOUT, REQUEST_TIMEOUT, UA};

mod r#const;
pub mod error;
pub mod model;
pub mod scan;

pub type ScanResult<T, E = QrCodeScannerError> = anyhow::Result<T, E>;
pub type DriveResult<T, E = DriveError> = anyhow::Result<T, E>;

pub struct Drive {
    client: reqwest::Client,
}

impl Drive {
    pub async fn new() -> DriveResult<Self> {
        let client = reqwest::Client::builder()
            .user_agent(UA)
            .pool_idle_timeout(REQUEST_POOL_IDLE_TIMEOUT)
            .connect_timeout(REQUEST_CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()?;
        Ok(Self { client })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
