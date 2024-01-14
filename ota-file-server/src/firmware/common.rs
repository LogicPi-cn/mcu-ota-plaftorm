use std::fmt;

use serde::{Deserialize, Serialize};

// 固件版本
#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct FirmwareVersion {
    pub m: u8,
    pub n: u8,
    pub l: u8,
}

// 固件信息
#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct FirmwareInfo {
    pub code: u16,
    pub version: FirmwareVersion,
    pub size: u32,    // 以字节为单位
    pub path: String, // 文件路径
}

// 打印
impl fmt::Display for FirmwareInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FwInfo -> Code:{:04X}, Version: {}.{}.{}, Size: {} bytes",
            self.code, self.version.m, self.version.n, self.version.l, self.size
        )
    }
}
