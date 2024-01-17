use firmware::common::FirmwareVersion;
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::to_string;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取命令行参数
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: upload_firmware <code> <version> <file_path>");
        return Ok(());
    }
    let code = args[1].parse::<u16>()?;
    let version = parse_version(&args[2])?;
    let file_path = &args[3];

    // 读取固件文件内容
    let mut file_content = Vec::new();
    File::open(file_path)?.read_to_end(&mut file_content)?;

    // 创建一个 reqwest 的客户端
    let client = Client::new();

    // 构建 multipart/form-data 请求
    let form = Form::new()
        .part("code", Part::text(code.to_string()))
        .part("version", Part::text(to_string(&version)?))
        .part(
            "firmware",
            Part::bytes(file_content).file_name(
                Path::new(file_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            ),
        );

    // 发送 POST 请求
    let response = client
        .post("http://x.21up.cn:2000/firmware")
        .multipart(form)
        .send()
        .await?;

    // 检查响应状态码
    if response.status().is_success() {
        println!("Firmware uploaded successfully!");
    } else {
        println!("Failed to upload firmware. Status: {}", response.status());
    }

    Ok(())
}

fn parse_version(version_str: &str) -> Result<FirmwareVersion, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = version_str.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid version format".into());
    }

    let major = parts[0].parse::<u8>()?;
    let minor = parts[1].parse::<u8>()?;
    let patch = parts[2].parse::<u8>()?;

    Ok(FirmwareVersion {
        m: major,
        n: minor,
        l: patch,
    })
}
