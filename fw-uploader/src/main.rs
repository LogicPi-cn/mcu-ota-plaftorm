use base64::Engine;
use clap::Parser;
use firmware::models::firmware_data::{
    find_firmware, find_latest_fw, FirmwareVersion, NewFirmwareData, UpdateFirmwareData,
};

use chrono::Utc;
use fw_uploader::args::{Cli, Commands};
use fw_uploader::operation::{get_all_fw_datas, push_new_firmware, update_firmware};
use log::info;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use base64::engine::general_purpose;
use tokio::runtime::Runtime;

fn main() {
    // set log level
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_LOG");

    // Get Parameters
    let cli = Cli::parse();

    let sever = cli.server.clone();

    match cli.command {
        Commands::List { fw_code } => {
            // 创建并运行异步任务
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let all_fw_files = get_all_fw_datas(&sever.clone()).await;

                let fwcode = i32::from_str_radix(&fw_code.clone(), 16).unwrap();

                match find_latest_fw(&all_fw_files, fwcode) {
                    Some(old_fw) => {
                        info!("Lates Firmware : {}", old_fw);
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
            let fwcode = i32::from_str_radix(&fw_code.clone(), 16).unwrap();
            let version = fw_version.clone();
            let file_path = fw_path.clone();

            let parts: Vec<&str> = version.split('.').collect();
            let version_m = parts[0].parse::<i32>().expect("Invalid version number");
            let version_n = parts[1].parse::<i32>().expect("Invalid version number");
            let version_l = parts[2].parse::<i32>().expect("Invalid version number");

            // 读取文件内容
            let mut buf = Vec::new();
            let mut file = File::open(&Path::new(&fw_path.clone())).expect("Failed to open file");
            file.read_to_end(&mut buf).expect("Failed to read file");

            // 获取文件大小
            let fwsize = std::fs::metadata(file_path)
                .expect("Failed to get file metadata")
                .len() as i32;

            let _version = FirmwareVersion {
                m: version_m,
                n: version_n,
                l: version_l,
            };

            // 创建 FirmwareData 实例
            let _new_data = NewFirmwareData {
                fwcode,
                version_m,
                version_n,
                version_l,
                fwsize,
                fwdata: general_purpose::STANDARD.encode(&buf).into(),
            };

            // 创新待更新的UpdateFirmwareData
            let _updated_fw = UpdateFirmwareData {
                fwcode,
                version_m,
                version_n,
                version_l,
                fwsize,
                fwdata: general_purpose::STANDARD.encode(&buf).into(),
                updated_at: Some(Utc::now().naive_utc()),
            };

            // 创建并运行异步任务
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let all_fw_files = get_all_fw_datas(&sever.clone()).await;

                match find_firmware(&all_fw_files, fwcode, _version) {
                    Some(old_fw) => {
                        // 如果已有固件了，则更新固件
                        info!("Firmware existed, updating... {}", _updated_fw);
                        update_firmware(&sever.clone(), old_fw.id, &_updated_fw).await;
                    }
                    None => {
                        // 如果没有找到固件，则新上传
                        info!("Upload a new firmware -> {}", _new_data);
                        push_new_firmware(&sever.clone(), &_new_data).await;
                    }
                }
            });
        }
    }
}
