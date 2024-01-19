use base64::Engine;
use clap::Parser;
use firmware::models::firmware_data::NewFirmwareData;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use user_client::args::Cli;

use base64::engine::general_purpose;
use tokio::runtime::Runtime;

fn main() {
    // Get Parameters
    let cli = Cli::parse();

    let sever = cli.server.clone();
    let fwcode = cli.fw_code.clone();
    let version = cli.fw_version.clone();
    let file_path = cli.fw_path.clone();

    let parts: Vec<&str> = version.split('.').collect();
    let version_m = parts[0].parse::<i32>().expect("Invalid version number");
    let version_n = parts[1].parse::<i32>().expect("Invalid version number");
    let version_l = parts[2].parse::<i32>().expect("Invalid version number");

    // 读取文件内容
    let mut buf = Vec::new();
    let mut file = File::open(&Path::new(&cli.fw_path.clone())).expect("Failed to open file");
    file.read_to_end(&mut buf).expect("Failed to read file");

    // 获取文件大小
    let fwsize = std::fs::metadata(file_path)
        .expect("Failed to get file metadata")
        .len() as i32;

    // 创建 FirmwareData 实例
    let data = NewFirmwareData {
        fwcode,
        version_m,
        version_n,
        version_l,
        fwsize,
        fwdata: general_purpose::STANDARD.encode(&buf).into(),
    };

    // 创建并运行异步任务
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = reqwest::Client::new();

        // 检查是否有固件
        // let res = client.get(&sever.clone()).send().await;
        // match res {
        //     Ok(response) => {
        //         if response.status().is_success() {
        //             println!("Firmware uploaded successfully");
        //         } else {
        //             println!("Failed to upload firmware: {}", response.status());
        //         }
        //     }
        //     Err(e) => {
        //         println!("Failed to find firmware: {}", e);
        //     }
        // }

        // 发送 POST 请求
        let res = client.post(&sever.clone()).json(&data).send().await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Firmware uploaded successfully");
                } else {
                    println!("Failed to upload firmware: {}", response.status());
                }
            }
            Err(e) => {
                println!("Failed to upload firmware: {}", e);
            }
        }
    });
}
