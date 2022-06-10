use crate::error::{DriveError, QrCodeScannerError};

pub mod error;
pub mod scan;

pub type ScanResult<T, E = QrCodeScannerError> = anyhow::Result<T, E>;
pub type DriveResult<T, E = DriveError> = anyhow::Result<T, E>;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
