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

// 固件查询
pub const PKG_RX_FW_INFO: u8 = 0xA1;
pub const PKG_TX_FW_INFO: u8 = 0xFF - PKG_RX_FW_INFO;

// 固件下载
pub const PKG_RX_FW_DATA: u8 = 0xA2;
pub const PKG_TX_FW_DATA: u8 = 0xFF - PKG_RX_FW_DATA;

// 下载结束
pub const PKG_RX_FW_END: u8 = 0xA3;
pub const PKG_TX_FW_END: u8 = 0xFF - PKG_RX_FW_END;

// 错误码
pub const PKG_FAILED_CRC: u8 = 0xF0; // CRC错误
pub const PKG_FAILED_LEN: u8 = 0xF1; // 长度不对
pub const PKG_FAILED_NO_FW_FOUND: u8 = 0xF2; // 找不到此固件
pub const PKG_FAILED_FW_READ_ERROR: u8 = 0xF3; // 找不到此固件
