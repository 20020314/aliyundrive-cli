use crate::error::{DriveError, QrCodeScannerError};
use crate::standard::{REQUEST_CONNECT_TIMEOUT, REQUEST_POOL_IDLE_TIMEOUT, REQUEST_TIMEOUT, UA};
use serde::{Deserialize, Deserializer, Serialize};

pub mod conf;
pub mod error;
pub mod model;
pub mod scan;
pub mod standard;

pub type ScanResult<T, E = QrCodeScannerError> = anyhow::Result<T, E>;
pub type DriveResult<T, E = DriveError> = anyhow::Result<T, E>;

#[derive(Serialize, Debug, Clone)]
pub struct DateTime(String);

impl DateTime {
    pub fn new(st: String) -> Self {
        Self(st)
    }
}

impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let result = <&str>::deserialize(deserializer)?;
        let dt = chrono::DateTime::parse_from_rfc3339(result).map_err(serde::de::Error::custom)?;
        let format = dt
            .with_timezone(&chrono::FixedOffset::east(8 * 3600))
            .format(standard::TIME_FORMAT);
        Ok(DateTime::new(format.to_string()))
    }
}

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
