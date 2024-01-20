use firmware::models::firmware_data::{
    find_firmware, find_latest_fw, slice_fw_data_from_vector, FirmwareData, FirmwareInfo,
    FirmwareVersion,
};
use log::{debug, error, info};
use std::{error::Error, sync::Arc};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};

use crate::{
    package::{common::package_check, tx_package::*},
    PKG_FAILED_CRC, PKG_FAILED_FW_READ_ERROR, PKG_FAILED_LEN, PKG_FAILED_NO_FW_FOUND,
    PKG_RX_FW_DATA, PKG_RX_FW_END, PKG_RX_FW_INFO,
};

/// 处理tcp请求入口
pub async fn handle_client(
    mut socket: TcpStream,
    fw_data_all: Arc<Mutex<Vec<FirmwareData>>>,
) -> Result<(), Box<dyn Error>> {
    info!("New client connected: {:?}", socket.peer_addr()?);

    let mut buffer = [0; 1024];

    loop {
        // 从客户端读取数据
        let bytes_read = socket.read(&mut buffer).await.unwrap();

        if bytes_read == 0 {
            // 客户端关闭连接
            break;
        }

        // 处理接收到的数据
        let request = &buffer[..bytes_read].to_vec();
        let fw_data_all = fw_data_all.lock().await;

        package_process(request, &mut socket, &fw_data_all).await?;

        // 清空缓冲区
        buffer.fill(0);
    }

    info!("Client disconnected: {:?}", socket.peer_addr());

    Ok(())
}

async fn package_process(
    request: &Vec<u8>,
    socket: &mut TcpStream,
    fw_data_all: &Vec<FirmwareData>,
) -> Result<(), Box<dyn Error>> {
    // 最低长度为7
    if request.len() >= 7 {
        // CRC检查
        if package_check(&request, request.len()) {
            // 固件代号
            let _code = (request[5] as u16) << 8 | request[6] as u16;

            // 固件查询指令
            if request[2] == PKG_RX_FW_INFO {
                proces_fw_query_request(request, socket, _code as i32, &fw_data_all).await?;
            }

            // 固件查询指令
            if request[2] == PKG_RX_FW_DATA {
                proces_fw_download_request(request, socket, _code as i32, &fw_data_all).await?;
            }

            // 下载结束指令
            if request[2] == PKG_RX_FW_END {
                proces_fw_end_request(request, socket, _code as i32, &fw_data_all).await?;
            }
        } else {
            // CRC失败
            error!("Package CRC Error!");
            send_failed_package(socket, PKG_FAILED_CRC).await?;
        }
    } else {
        // 长度不对
        error!("Package Length Error!");
        send_failed_package(socket, PKG_FAILED_LEN).await?;
    }

    Ok(())
}

// 处理固件查询请求
async fn proces_fw_query_request(
    _request: &Vec<u8>,
    socket: &mut TcpStream,
    code: i32,
    fw_data_all: &Vec<FirmwareData>,
) -> Result<(), Box<dyn Error>> {
    debug!("[Command] Query Firmware Info.");
    if let Some(fw_data) = find_latest_fw(&fw_data_all, code) {
        send_fw_info(
            &FirmwareInfo {
                code: fw_data.fwcode,
                version: FirmwareVersion {
                    m: fw_data.version_m,
                    n: fw_data.version_n,
                    l: fw_data.version_l,
                },
                size: fw_data.fwsize,
                path: String::from(""),
            },
            socket,
        )
        .await?;
    } else {
        error!("No firmware found!");
        send_failed_package(socket, PKG_FAILED_NO_FW_FOUND).await?;
    }

    Ok(())
}

// 处理固件下载请求
async fn proces_fw_download_request(
    request: &Vec<u8>,
    socket: &mut TcpStream,
    _code: i32,
    fw_data_all: &Vec<FirmwareData>,
) -> Result<(), Box<dyn Error>> {
    debug!("[Command] Download Firmware.");

    let _version: FirmwareVersion = FirmwareVersion {
        m: request[7] as i32,
        n: request[8] as i32,
        l: request[9] as i32,
    };

    let _index = (request[10] as u16) << 8 | request[11] as u16; // 切片序号
    let _slice = (request[12] as u16) << 8 | request[13] as u16; // 切片大小，一般默认512

    if let Some(fw_data) = find_firmware(fw_data_all, _code as i32, _version) {
        let data = slice_fw_data_from_vector(&fw_data.fwdata, _index as usize, _slice as usize);

        match data {
            Some(data) => {
                // 发送固件数据
                debug!(
                    "Sending Firmware Data -> index:{}, slice:{}, len:{}",
                    _index,
                    _slice,
                    data.len()
                );
                send_fw_data(
                    &FirmwareInfo {
                        code: fw_data.fwcode,
                        version: FirmwareVersion {
                            m: fw_data.version_m,
                            n: fw_data.version_n,
                            l: fw_data.version_l,
                        },
                        size: fw_data.fwsize,
                        path: String::from(""),
                    },
                    &data,
                    _index,
                    socket,
                )
                .await?;
            }
            None => {
                // 发送文件错误
                debug!("Read Firmware Error!");
                send_failed_package(socket, PKG_FAILED_FW_READ_ERROR).await?;
            }
        }
    } else {
        error!("No firmware found!");
        send_failed_package(socket, PKG_FAILED_NO_FW_FOUND).await?;
    }

    Ok(())
}

// 处理固件结束请求
async fn proces_fw_end_request(
    request: &Vec<u8>,
    socket: &mut TcpStream,
    _code: i32,
    fw_data_all: &Vec<FirmwareData>,
) -> Result<(), Box<dyn Error>> {
    debug!("[Command] Download Firmware Over.");

    let _version: FirmwareVersion = FirmwareVersion {
        m: request[7] as i32,
        n: request[8] as i32,
        l: request[9] as i32,
    };

    let _index = (request[10] as u16) << 8 | request[11] as u16; // 切片序号
    let _slice = (request[12] as u16) << 8 | request[13] as u16; // 切片大小，一般默认512

    if let Some(fw_data) = find_firmware(fw_data_all, _code, _version) {
        send_fw_end(
            &FirmwareInfo {
                code: fw_data.fwcode,
                version: FirmwareVersion {
                    m: fw_data.version_m,
                    n: fw_data.version_n,
                    l: fw_data.version_l,
                },
                size: fw_data.fwsize,
                path: String::from(""),
            },
            socket,
        )
        .await?;
    } else {
        error!("No firmware found!");
        send_failed_package(socket, PKG_FAILED_NO_FW_FOUND).await?;
    }

    Ok(())
}
