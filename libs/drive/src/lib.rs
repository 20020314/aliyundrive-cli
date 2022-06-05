use crate::error::{DriveError, QrCodeScannerError};

pub mod error;
pub mod scan;

pub type ScanResult<T, E = QrCodeScannerError> = Result<T, E>;
pub type DriveResult<T, E = DriveError> = Result<T, E>;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
