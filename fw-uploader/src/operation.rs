use log::info;
use ota_database::models::firmware_data::{FirmwareData, NewFirmwareData, UpdateFirmwareData};

/// 更新固件
pub async fn update_firmware(server: &str, id: i32, updated_fw: &UpdateFirmwareData) {
    let client = reqwest::Client::new();
    let res = client
        .patch(format!("{}/{}", server, id))
        .json(&updated_fw)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                info!("Firmware updated successfully");
            } else {
                info!("Failed to update firmware: {}", response.status());
            }
        }
        Err(e) => {
            info!("Failed to update firmware: {}", e);
        }
    }
}

/// 上传固件
pub async fn push_new_firmware(server: &str, new_data: &NewFirmwareData) {
    let client = reqwest::Client::new();
    let res = client.post(server).json(&new_data).send().await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                info!("Firmware uploaded successfully");
            } else {
                info!("Failed to upload firmware: {}", response.status());
            }
        }
        Err(e) => {
            info!("Failed to upload firmware: {}", e);
        }
    }
}

/// 获取现有的固件
pub async fn get_all_fw_datas(server: &str) -> Vec<FirmwareData> {
    let client = reqwest::Client::new();
    let mut all_fw_datas: Vec<FirmwareData> = Vec::new();

    let res = client.get(server).send().await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let result: Vec<FirmwareData> = response.json().await.unwrap();
                for data in result {
                    all_fw_datas.push(data);
                }
            } else {
                println!("Failed to get firmware: {}", response.status());
            }
        }
        Err(e) => {
            println!("Failed to find firmware: {}", e);
        }
    }

    all_fw_datas
}
