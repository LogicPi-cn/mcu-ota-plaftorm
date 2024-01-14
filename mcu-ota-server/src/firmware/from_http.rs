use clap::Parser;
use log::{debug, error};
use reqwest::Error;
use std::{env, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{self, Duration},
};

use crate::args::Cli;

use super::common::{FirmwareData, FirmwareInfo};

/// http下载所有固件
pub async fn http_get_all_fw_data(fw_server: &str) -> Result<Vec<FirmwareData>, Error> {
    let client = reqwest::Client::new();
    let response = client.get(format!("{}/list", fw_server)).send().await;

    let mut fw_data: Vec<FirmwareData> = Vec::new();

    match response {
        Ok(response) => {
            let firmware_infos: Vec<FirmwareInfo> = response.json().await?;
            debug!("Found {} firmware files.", firmware_infos.len());

            for fw_info in firmware_infos {
                debug!("Downloading... {}", fw_info);
                let client = reqwest::Client::new();
                let response = client
                    .get(format!(
                        "{}/download/{}",
                        fw_server,
                        &fw_info.clone().filename()
                    ))
                    .send()
                    .await;

                match response {
                    Ok(response) => {
                        let bytes = response.bytes().await?;

                        let new_data = FirmwareData {
                            info: fw_info.clone(),
                            data: bytes.to_vec(),
                        };

                        fw_data.push(new_data);
                    }
                    Err(e) => {
                        debug!("Error:{}", e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Error:{}, fw_server={}", e, fw_server);
        }
    }

    Ok(fw_data)
}

/// 定时刷新固件数据
/// ## 参数
/// - fw_server   : 服务器地址
/// - min         : 刷新周期，以分钟为单位
/// - fw_data_all : 原子变量，存放所有固件数据
pub async fn refresh_firmware_data(fw_data_all: Arc<Mutex<Vec<FirmwareData>>>) {
    let cli = Cli::parse();

    let fw_server = env::var("FW_SERVER").unwrap_or_else(|_| cli.clone().fw_server);

    // 刷新周期
    let refresh_duration = Duration::from_secs(1 * 60);

    loop {
        // 读取固件数据
        let new_data = http_get_all_fw_data(&fw_server).await;

        match new_data {
            Ok(new_data) => {
                // 更新全局变量
                let mut fw_data_all = fw_data_all.lock().await;
                *fw_data_all = new_data;

                debug!("Refreshed");
            }
            Err(e) => {
                error!("Error:{}", e);
            }
        }
        time::sleep(refresh_duration).await;
    }
}

/// 读取固件数据
pub async fn read_firmware_data(fw_data_all: Arc<Mutex<Vec<FirmwareData>>>) -> Vec<FirmwareData> {
    let fw_data_all = fw_data_all.lock().await;
    fw_data_all.clone() // 注意，这里我们返回了数据的一份克隆
}
