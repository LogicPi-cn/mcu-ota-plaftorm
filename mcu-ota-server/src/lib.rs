pub mod args;
pub mod package;
pub mod process_pg;

#[derive(Debug, Clone, Copy)]
pub enum PackageType {
    FirmwareQuery = 0xA1,
    FirmwareDownload = 0xA2,
    DownloadEnd = 0xA3,
}

impl PackageType {
    pub fn to_response(&self) -> u8 {
        0xFF - *self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    CrcError = 0xF0,
    LengthError = 0xF1,
    NoFirmwareFound = 0xF2,
    FirmwareReadError = 0xF3,
    UnknownPackageType = 0xF4,
}
