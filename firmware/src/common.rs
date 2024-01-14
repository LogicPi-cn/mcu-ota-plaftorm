use serde::{Deserialize, Serialize};
use std::fmt;

/// 固件版本结构体
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FirmwareVersion {
    pub m: u8,
    pub n: u8,
    pub l: u8,
}

/// 固件信息
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FirmwareInfo {
    pub code: u16,
    pub version: FirmwareVersion,
    pub size: u32,    // 以字节为单位
    pub path: String, // 文件路径
}

/// 固件数据
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FirmwareData {
    pub info: FirmwareInfo, // 固件信息
    pub data: Vec<u8>,      // 固件数据
}

/// 文件名
impl FirmwareInfo {
    /// * 根据FirmwareInfo生成bin文件名
    /// * 生成文件名格式： 1987-0.2.0.bin
    pub fn filename(self) -> String {
        format!(
            "{:04X}-{}.{}.{}.bin",
            self.code, self.version.m, self.version.n, self.version.l
        )
    }
}

/// 格式化打印
impl fmt::Display for FirmwareInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FwInfo -> Code:{:04X}, Version: {}.{}.{}, Size: {} bytes",
            self.code, self.version.m, self.version.n, self.version.l, self.size
        )
    }
}

/// 根据code查找最新版本的固件
pub fn find_latest_fw(all_fw_files: &[FirmwareData], code: u16) -> Option<FirmwareData> {
    let filtered_fw_files: Vec<&FirmwareData> = all_fw_files
        .iter()
        .filter(|fw| fw.info.code == code)
        .collect();

    filtered_fw_files
        .into_iter()
        .max_by(|a, b| {
            let version_a = &a.info.version;
            let version_b = &b.info.version;
            (version_a.m, version_a.n, version_a.l).cmp(&(version_b.m, version_b.n, version_b.l))
        })
        .cloned()
}

/// 根据code和version查找具体固件
pub fn find_firmware(
    all_fw_files: &[FirmwareData],
    code: u16,
    version: FirmwareVersion,
) -> Option<FirmwareData> {
    all_fw_files
        .iter()
        .find(|firmware| firmware.info.code == code && firmware.info.version == version)
        .cloned()
}

/// 切片固件数据
pub fn slice_fw_data_from_vector(data: &[u8], index: usize, slice_size: usize) -> Option<Vec<u8>> {
    let start_position = index * slice_size;

    // 先判断start_position有没有越界
    if start_position >= data.len() {
        return None;
    }

    // 再判断end_position有没有越界
    let end_position = std::cmp::min(start_position + slice_size, data.len());

    if end_position <= start_position {
        return None;
    }

    Some(data[start_position..end_position].to_vec())
}
