use crc::{Crc, CRC_8_MAXIM_DOW};
use ota_database::models::{config_history::ConfigHistory, firmware_data::FirmwareInfo};
use std::error::Error;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::PackageType;

use super::common::datetime_to_vec;

/// 发送失败数据包
pub async fn send_failed_package(
    socket: &mut TcpStream,
    failed_code: u8,
) -> Result<(), Box<dyn Error>> {
    let response = gen_failed_package(failed_code);
    send_response_package(&response, socket).await?;
    Ok(())
}

/// 发送固件信息
pub async fn send_fw_info(
    fw_info: &FirmwareInfo,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    let response = gen_fw_info_package(&fw_info);
    send_response_package(&response, socket).await?;
    Ok(())
}

/// 发送固件数据
pub async fn send_fw_data(
    fw_info: &FirmwareInfo,
    data: &Vec<u8>,
    index: u16,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    let response = gen_fw_data_package(&fw_info, data, index);
    send_response_package(&response, socket).await?;
    Ok(())
}

/// 发送固件结束包
pub async fn send_fw_end(
    fw_info: &FirmwareInfo,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    let response = gen_fw_end_package(&fw_info);
    send_response_package(&response, socket).await?;
    Ok(())
}

/// 发送配置数据
pub async fn send_config(
    last_config: &ConfigHistory,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    let response = gen_config_package(last_config);
    send_response_package(&response, socket).await?;
    Ok(())
}

/// 发送返回包   Server->MCU
async fn send_response_package(
    package: &Vec<u8>,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    // 返回数据包
    socket.write_all(&package).await?;
    // 确保数据立即发送
    socket.flush().await?;
    Ok(())
}

/// 生成配置数据包
fn gen_config_package(last_config: &ConfigHistory) -> Vec<u8> {
    let data_vec = datetime_to_vec(last_config.sync_ts);

    let mut data: Vec<u8> = vec![
        0xAA,
        0x55,
        PackageType::QueryConfig.to_response(), // 配置查询
        0x00,                                   // 数据长度
        0x0E,                                   // 数据长度
        last_config.group_id as u8,             // 分组号
        last_config.op_code as u8,              // 操作码
        data_vec[0],                            // 日期
        data_vec[1],                            // 日期
        data_vec[2],                            // 日期
        data_vec[3],                            // 日期
        data_vec[4],                            // 日期
        data_vec[5],                            // 日期
        last_config.interval as u8,             // 时间间隔
        (last_config.t_max >> 8) as u8,         // 温度上限
        (last_config.t_max & 0xFF) as u8,       // 温度上限
        (last_config.t_min >> 8) as u8,         // 温度下限
        (last_config.t_min & 0xFF) as u8,       // 温度下限
        if last_config.human { 0xA0 } else { 0xA1 }, // 人体状态
    ];

    // 计算crc
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let crc = crc8_checksum.checksum(&data);

    // 添加CRC
    data.push(crc);

    data
}

/// 错误包生成
fn gen_failed_package(failed_code: u8) -> Vec<u8> {
    // 包头
    let mut data: Vec<u8> = vec![0xAA, 0x55, failed_code];

    // 计算crc
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let crc = crc8_checksum.checksum(&data);

    // 添加CRC
    data.push(crc);

    data
}

/// 生成固件信息包
fn gen_fw_info_package(fw_info: &FirmwareInfo) -> Vec<u8> {
    // 包头
    let mut data: Vec<u8> = vec![
        0xAA,
        0x55,                                     // 包头
        PackageType::FirmwareQuery.to_response(), // 包类型：固件查询
        0x00,                                     // 长度
        0x09,                                     // 长度
        (fw_info.code >> 8) as u8,                // Code 高8位
        (fw_info.code & 0xFF) as u8,              // Code 低8位
        fw_info.version.m as u8,                  // 版本号 大
        fw_info.version.n as u8,                  // 版本号 中
        fw_info.version.l as u8,                  // 版本号 小
        (fw_info.size >> 24) as u8,
        (fw_info.size >> 16) as u8,
        (fw_info.size >> 8) as u8,
        fw_info.size as u8,
    ];

    // 计算crc
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let crc = crc8_checksum.checksum(&data);

    // 添加CRC
    data.push(crc);

    data
}

/// 生成固件数据包
fn gen_fw_data_package(fw_info: &FirmwareInfo, input_data: &Vec<u8>, index: u16) -> Vec<u8> {
    // 数据总长度
    let total_len = 7 + input_data.len();

    // 包头
    let mut data: Vec<u8> = vec![
        0xAA,
        0x55,                                        // 包头
        PackageType::FirmwareDownload.to_response(), // 包类型：固件查询
        (total_len >> 8) as u8,                      // Len 高8位
        (total_len & 0xFF) as u8,                    // Len 低8位
        (fw_info.code >> 8) as u8,                   // 固件代号 高8位
        (fw_info.code & 0xFF) as u8,                 // 固件代号 低8位
        fw_info.version.m as u8,                     // 固件版本号 大
        fw_info.version.n as u8,                     // 固件版本号 中
        fw_info.version.l as u8,                     // 固件版本号 小
        (index >> 8) as u8,                          // index 高8位
        (index & 0xFF) as u8,                        // index 低8位
    ];

    // 追加数据
    data.extend(input_data);

    // 计算crc
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let crc = crc8_checksum.checksum(&data);

    // 添加CRC
    data.push(crc);

    data
}

/// 生成固件结束包
fn gen_fw_end_package(fw_info: &FirmwareInfo) -> Vec<u8> {
    // 包头
    let mut data: Vec<u8> = vec![
        0xAA,
        0x55,                                   // 包头
        PackageType::DownloadEnd.to_response(), // 包类型：结束
        0x00,                                   // 长度
        0x05,                                   // 长度
        (fw_info.code >> 8) as u8,              // Code 高8位
        (fw_info.code & 0xFF) as u8,            // Code 低8位
        fw_info.version.m as u8,                // 版本号 大
        fw_info.version.n as u8,                // 版本号 中
        fw_info.version.l as u8,                // 版本号 小
    ];

    // 计算crc
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let crc = crc8_checksum.checksum(&data);

    // 添加CRC
    data.push(crc);

    data
}
