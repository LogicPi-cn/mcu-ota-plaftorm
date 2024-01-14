use std::{
    fs::{self, File},
    io::{self, Read, Seek},
};

use clap::Parser;
use log::debug;
use regex::Regex;

use crate::args::Cli;

use super::common::{FirmwareInfo, FirmwareVersion};

// 查询本地固件，然后返回版本和固件大小
pub fn get_firmware_info(code_to_find: u16) -> Option<FirmwareInfo> {
    let cli = Cli::parse();

    let dir_path = cli.fw_path;
    let all_fw_files: Vec<FirmwareInfo> = list_all_fw(&dir_path);

    if let Some(latest_fw) = find_latest_firmware(&all_fw_files, code_to_find) {
        debug!("{}", latest_fw);
        Some(latest_fw)
    } else {
        debug!("No firmware found for code: {:04X}", code_to_find);
        None
    }
}

// 根据code和versions查找固件
pub fn find_firmware(code: u16, version: FirmwareVersion) -> Option<FirmwareInfo> {
    let cli = Cli::parse();
    let dir_path = cli.fw_path;
    let all_fw_files: Vec<FirmwareInfo> = list_all_fw(&dir_path);

    all_fw_files
        .iter()
        .find(|firmware| firmware.code == code && firmware.version == version)
        .cloned()
}

// 根据code查找最新版本的固件
fn find_latest_firmware(all_fw_files: &[FirmwareInfo], code: u16) -> Option<FirmwareInfo> {
    let filtered_fw_files: Vec<&FirmwareInfo> =
        all_fw_files.iter().filter(|fw| fw.code == code).collect();

    filtered_fw_files
        .into_iter()
        .max_by(|a, b| {
            let version_a = &a.version;
            let version_b = &b.version;
            (version_a.m, version_a.n, version_a.l).cmp(&(version_b.m, version_b.n, version_b.l))
        })
        .cloned()
}

// 遍历目录
pub fn list_all_fw(path: &str) -> Vec<FirmwareInfo> {
    let dir_path = path;
    let mut firmware_infos: Vec<FirmwareInfo> = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        let re = Regex::new(r"(\d+)-(\d+\.\d+\.\d+)\.bin").unwrap();

        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy().to_string();

                if let Some(captures) = re.captures(&file_name_str) {
                    // let code = captures[1].parse::<u16>().unwrap();
                    let code_str = captures[1].to_string();
                    let code = u16::from_str_radix(&code_str, 16).unwrap();
                    let version_parts: Vec<&str> = captures[2].split('.').collect();
                    if version_parts.len() == 3 {
                        let version = FirmwareVersion {
                            m: version_parts[0].parse::<u8>().unwrap(),
                            n: version_parts[1].parse::<u8>().unwrap(),
                            l: version_parts[2].parse::<u8>().unwrap(),
                        };
                        let size = entry.metadata().unwrap().len() as u32;

                        let firmware_info = FirmwareInfo {
                            code,
                            version,
                            size,
                            path: format!("{}/{}", dir_path, file_name_str),
                        };

                        firmware_infos.push(firmware_info);
                    }
                }
            }
        }
    } else {
        debug!("Failed to read directory");
    }

    // 打印固件信息
    // for firmware_info in &firmware_infos {
    //     info!("{}", firmware_info);
    // }

    firmware_infos
}

// 切片固件数据
pub fn slice_firmware_file(path: &str, index: usize, slice_size: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0; slice_size];
    let start_position = index * slice_size;

    file.seek(io::SeekFrom::Start(start_position as u64))?;
    let bytes_read = file.read(&mut buffer)?;

    if bytes_read < slice_size {
        buffer.truncate(bytes_read);
    }

    Ok(buffer)
}
