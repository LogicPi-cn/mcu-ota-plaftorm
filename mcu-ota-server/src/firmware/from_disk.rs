use std::{
    fs::{self, File},
    io::{self, Read, Seek},
};

use log::debug;
use regex::Regex;

use super::common::{FirmwareInfo, FirmwareVersion};

/// 遍历目录，返回所有固件清单
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

    firmware_infos
}

/// 切片固件数据
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
