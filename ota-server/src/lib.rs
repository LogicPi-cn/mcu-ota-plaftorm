pub mod args;
pub mod package;
pub mod process_pg;

/// LogicPi Logo
pub const LOGO: &str = r"
    __    ____   ______ ____ ______ ____   ____
   / /   / __ \ / ____//  _// ____// __ \ /  _/
  / /   / / / // / __  / / / /    / /_/ / / /
 / /___/ /_/ // /_/ /_/ / / /___ / ____/_/ /
/_____/\____/ \____//___/ \____//_/    /___/
";

#[derive(Debug, Clone, Copy)]
pub enum PackageType {
    FirmwareQuery = 0xA1,    // 固件查询
    FirmwareDownload = 0xA2, // 固件下载
    DownloadEnd = 0xA3,      // 下载结束
    QueryConfig = 0xA4,      // 参数查询
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
