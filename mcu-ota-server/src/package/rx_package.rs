use std::error::Error;

use crc::{Crc, CRC_8_MAXIM_DOW};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{firmware::common::FirmwareInfo, PKG_RX_FW_DATA, PKG_RX_FW_END, PKG_RX_FW_INFO};

// 发送固件信息
pub async fn send_fw_query_info_pkg(
    fw_info: &FirmwareInfo,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    let response = gen_fw_query_info_package(&fw_info);
    send_query_package(&response, socket).await?;
    Ok(())
}

// 发送固件数据
pub async fn send_fw_query_data_pkg(
    fw_info: &FirmwareInfo,
    index: u16,
    slice: u16,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    let response = gen_fw_query_data_package(&fw_info, index, slice);
    send_query_package(&response, socket).await?;
    Ok(())
}

// 发送固件结束包
pub async fn send_fw_query_end_pkg(
    fw_info: &FirmwareInfo,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    let response = gen_fw_query_end_package(&fw_info);
    send_query_package(&response, socket).await?;
    Ok(())
}

// 发送返回包   Server->MCU
async fn send_query_package(
    package: &Vec<u8>,
    socket: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    // 返回数据包
    socket.write_all(&package).await?;
    // 确保数据立即发送
    socket.flush().await?;
    Ok(())
}

// 固件信息请求包
fn gen_fw_query_info_package(fw_info: &FirmwareInfo) -> Vec<u8> {
    // 包头
    let mut data: Vec<u8> = vec![
        0xAA,
        0x55,                        // 包头
        PKG_RX_FW_INFO,              // 包类型：固件查询
        0x00,                        // 长度
        0x02,                        // 长度
        (fw_info.code >> 8) as u8,   // Code 高8位
        (fw_info.code & 0xFF) as u8, // Code 低8位
    ];

    // 计算crc
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let crc = crc8_checksum.checksum(&data);

    // 添加CRC
    data.push(crc);

    data
}

// 固件下载请求包
fn gen_fw_query_data_package(fw_info: &FirmwareInfo, index: u16, slice: u16) -> Vec<u8> {
    // 包头
    let mut data: Vec<u8> = vec![
        0xAA,
        0x55,                        // 包头
        PKG_RX_FW_DATA,              // 包类型：固件请求
        0x00,                        // Len 高8位
        0x09,                        // Len 低8位
        (fw_info.code >> 8) as u8,   // 固件代号 高8位
        (fw_info.code & 0xFF) as u8, // 固件代号 低8位
        fw_info.version.m,           // 固件版本号 大
        fw_info.version.n,           // 固件版本号 中
        fw_info.version.l,           // 固件版本号 小
        (index >> 8) as u8,          // index 高8位
        (index & 0xFF) as u8,        // index 低8位
        (slice >> 8) as u8,          // slice 高8位
        (slice & 0xFF) as u8,        // slice 低8位
    ];

    // 计算crc
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let crc = crc8_checksum.checksum(&data);

    // 添加CRC
    data.push(crc);

    data
}

// 请求固件结束包
fn gen_fw_query_end_package(fw_info: &FirmwareInfo) -> Vec<u8> {
    // 包头
    let mut data: Vec<u8> = vec![
        0xAA,
        0x55,                        // 包头
        PKG_RX_FW_END,               // 包类型：结束
        0x00,                        // 长度
        0x05,                        // 长度
        (fw_info.code >> 8) as u8,   // Code 高8位
        (fw_info.code & 0xFF) as u8, // Code 低8位
        fw_info.version.m,           // 版本号 大
        fw_info.version.n,           // 版本号 中
        fw_info.version.l,           // 版本号 小
    ];

    // 计算crc
    let crc8_checksum: Crc<u8> = Crc::<u8>::new(&CRC_8_MAXIM_DOW);
    let crc = crc8_checksum.checksum(&data);

    // 添加CRC
    data.push(crc);

    data
}
