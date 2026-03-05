use clap::Parser;
use ota_database::models::firmware_data::{
    find_firmware, find_latest_fw, FirmwareVersion, NewFirmwareData, UpdateFirmwareData,
};

use chrono::Utc;
use fw_uploader::args::{Cli, Commands};
use fw_uploader::operation::{get_all_fw_datas, push_new_firmware, update_firmware};
use log::{error, info};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokio::runtime::Runtime;

fn main() {
    // set log level
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_LOG");

    // Get Parameters
    let cli = Cli::parse();

    let server = cli.server.clone();

    match cli.command {
        Commands::List { fw_code } => {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(async {
                let all_fw_files = get_all_fw_datas(&server).await;

                let fwcode = match i32::from_str_radix(&fw_code, 16) {
                    Ok(code) => code,
                    Err(_) => {
                        error!("Invalid firmware code: {}", fw_code);
                        return;
                    }
                };

                match find_latest_fw(&all_fw_files, fwcode) {
                    Some(old_fw) => {
                        info!("Latest Firmware: {}", old_fw);
                    }
                    None => {
                        info!("Firmware Not Found!");
                    }
                }
            });
        }
        Commands::Upload {
            fw_code,
            fw_version,
            fw_path,
        } => {
            let fwcode = match i32::from_str_radix(&fw_code, 16) {
                Ok(code) => code,
                Err(_) => {
                    error!("Invalid firmware code: {}", fw_code);
                    return;
                }
            };

            let parts: Vec<&str> = fw_version.split('.').collect();
            let version_m = match parts[0].parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid version major number");
                    return;
                }
            };
            let version_n = match parts[1].parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid version minor number");
                    return;
                }
            };
            let version_l = match parts[2].parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    error!("Invalid version patch number");
                    return;
                }
            };

            // 读取文件内容
            let mut buf = Vec::new();
            let file_path = Path::new(&fw_path);
            let mut file = match File::open(file_path) {
                Ok(f) => f,
                Err(e) => {
                    error!("Failed to open file {}: {}", fw_path, e);
                    return;
                }
            };
            if let Err(e) = file.read_to_end(&mut buf) {
                error!("Failed to read file: {}", e);
                return;
            }

            // 获取文件大小
            let fwsize = buf.len() as i32;

            let version = FirmwareVersion {
                m: version_m,
                n: version_n,
                l: version_l,
            };

            // 创建 FirmwareData 实例 - 直接发送原始字节数据
            let new_data = NewFirmwareData {
                fwcode,
                version_m,
                version_n,
                version_l,
                fwsize,
                fwdata: buf.clone(),
            };

            // 创建待更新的 UpdateFirmwareData
            let updated_fw = UpdateFirmwareData {
                fwcode,
                version_m,
                version_n,
                version_l,
                fwsize,
                fwdata: buf,
                updated_at: Some(Utc::now().naive_utc()),
            };

            // 创建并运行异步任务
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(async {
                let all_fw_files = get_all_fw_datas(&server).await;

                match find_firmware(&all_fw_files, fwcode, version) {
                    Some(old_fw) => {
                        info!("Firmware existed, updating... {}", updated_fw);
                        update_firmware(&server, old_fw.id, &updated_fw).await;
                    }
                    None => {
                        info!("Upload a new firmware -> {}", new_data);
                        push_new_firmware(&server, &new_data).await;
                    }
                }
            });
        }
    }
}
