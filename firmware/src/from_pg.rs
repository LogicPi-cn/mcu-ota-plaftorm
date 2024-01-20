use std::sync::Arc;

use base64::{engine::general_purpose, Engine};
use log::{debug, error};
use reqwest::Error;
use tokio::{
    sync::Mutex,
    time::{self, Duration},
};

use crate::models::firmware_data::FirmwareData;

/// 从postgres数据库读取所有固件
async fn read_all_fw_from_pg(fw_server: &str) -> Result<Vec<FirmwareData>, Error> {
    let client = reqwest::Client::new();
    let response = client.get(format!("{}/firmware", fw_server)).send().await;

    let mut result_data: Vec<FirmwareData> = Vec::new();

    match response {
        Ok(response) => {
            let fw_datas: Vec<FirmwareData> = response.json().await?;
            debug!("Found {} firmware files.", fw_datas.len());

            for fw_data in fw_datas {
                debug!("Downloading... {}", fw_data);
                let new_data = FirmwareData {
                    id: fw_data.id,
                    fwcode: fw_data.fwcode,
                    version_m: fw_data.version_m,
                    version_n: fw_data.version_n,
                    version_l: fw_data.version_l,
                    fwdata: general_purpose::STANDARD.decode(&fw_data.fwdata).unwrap(),
                    fwsize: fw_data.fwsize,
                    created_at: fw_data.created_at,
                    updated_at: fw_data.updated_at,
                };

                result_data.push(new_data);
            }
        }
        Err(e) => {
            error!("Error:{}, fw_server={}", e, fw_server);
        }
    }

    Ok(result_data)
}

/// 定时刷新固件数据
/// ## 参数
/// - fw_server   : 服务器地址
/// - min         : 刷新周期，以分钟为单位
/// - fw_data_all : 原子变量，存放所有固件数据
pub async fn refresh_firmware_data(fw_server: &str, fw_data_all: Arc<Mutex<Vec<FirmwareData>>>) {
    // 刷新周期
    let refresh_duration = Duration::from_secs(1 * 60);

    loop {
        // 读取固件数据
        let new_data = read_all_fw_from_pg(&fw_server).await;

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
