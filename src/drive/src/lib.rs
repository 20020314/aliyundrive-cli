use crate::error::{DriveError, QrCodeScannerError};
use crate::scan::model::query::QueryQrCodeCkForm;
use crate::scan::model::{auth, AuthorizationToken, Ok};
use crate::standard::{REQUEST_CONNECT_TIMEOUT, REQUEST_POOL_IDLE_TIMEOUT, REQUEST_TIMEOUT, UA};
use anyhow::Context;
use chrono::{FixedOffset, TimeZone, Timelike, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::time::SystemTime;

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

    pub fn to_timestamp(&self) -> i64 {
        let time = chrono::NaiveDateTime::parse_from_str(self.0.as_str(), standard::TIME_FORMAT)
            .expect("Failed to format time");
        time.timestamp() - (8 * 3600)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let result = <&str>::deserialize(deserializer)?;

        let dt = chrono::DateTime::parse_from_rfc3339(result).map_err(serde::de::Error::custom)?;
        let format = dt
            .with_timezone(&FixedOffset::east(8 * 3600))
            .format(standard::TIME_FORMAT);
        Ok(DateTime::new(format.to_string()))
    }
}

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

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
