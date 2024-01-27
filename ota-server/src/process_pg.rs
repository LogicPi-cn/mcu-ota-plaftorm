use log::{debug, error, info};
use ota_database::{
    from_pg::{get_latest_config, read_config_from_pg},
    models::{
        firmware_data::{
            find_firmware, find_latest_fw, slice_fw_data_from_vector, FirmwareData, FirmwareInfo,
            FirmwareVersion,
        },
        upgrade_history::NewUpgradeHistory,
    },
};
use std::error::Error;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{
    package::{common::package_check, tx_package::*},
    ErrorCode, PackageType,
};

/// 处理tcp请求入口
pub async fn handle_client(
    mut socket: TcpStream,
    fw_data_all: &Vec<FirmwareData>,
    fw_server: &str,
) -> Result<(), Box<dyn Error>> {
    info!("New client connected: {:?}", socket.peer_addr()?);

    let mut buffer = [0; 1024];

    loop {
        // 从客户端读取数据
        let bytes_read = match socket.read(&mut buffer).await {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Socket Error :{}", e);
                break;
            }
        };

        // 客户端关闭连接
        if bytes_read == 0 {
            break;
        }

        // 处理接收到的数据
        let request = &buffer[..bytes_read].to_vec();

        package_process(request, &mut socket, &fw_data_all, &fw_server).await?;

        // 清空缓冲区
        buffer.fill(0);
    }

    info!("Client disconnected: {:?}", socket.peer_addr());

    Ok(())
}

/// 数据包处理入口
async fn package_process(
    request: &[u8],
    socket: &mut TcpStream,
    fw_data_all: &Vec<FirmwareData>,
    fw_server: &str,
) -> Result<(), Box<dyn Error>> {
    // 最低长度为7
    if request.len() >= 4 {
        // CRC检查
        if package_check(request, request.len()) {
            // 从请求中获取包类型
            let package_type = match request[2] {
                x if x == PackageType::FirmwareQuery as u8 => PackageType::FirmwareQuery,
                x if x == PackageType::FirmwareDownload as u8 => PackageType::FirmwareDownload,
                x if x == PackageType::DownloadEnd as u8 => PackageType::DownloadEnd,
                x if x == PackageType::QueryConfig as u8 => PackageType::QueryConfig,
                _ => {
                    error!("Unknown package type!");
                    send_failed_package(socket, ErrorCode::UnknownPackageType as u8).await?;
                    return Ok(());
                }
            };

            // 根据包类型处理请求
            match package_type {
                PackageType::FirmwareQuery => {
                    // 固件代号
                    let _code = (request[5] as u16) << 8 | request[6] as u16;
                    process_fw_query_request(request, socket, _code as i32, fw_data_all).await?
                }
                PackageType::FirmwareDownload => {
                    // 固件代号
                    let _code = (request[5] as u16) << 8 | request[6] as u16;
                    process_fw_download_request(request, socket, _code as i32, fw_data_all).await?
                }
                PackageType::DownloadEnd => {
                    // 固件代号
                    let _code = (request[5] as u16) << 8 | request[6] as u16;
                    process_fw_end_request(request, socket, _code as i32, fw_data_all).await?
                }
                PackageType::QueryConfig => {
                    process_query_config(request, socket, fw_server).await?
                }
            };
        } else {
            // CRC失败
            error!("Package CRC Error!");
            send_failed_package(socket, ErrorCode::CrcError as u8).await?;
        }
    } else {
        // 长度不对
        error!("Package Length Error!");
        send_failed_package(socket, ErrorCode::LengthError as u8).await?;
    }

    Ok(())
}

/// 配置查询
async fn process_query_config(
    _request: &[u8],
    socket: &mut TcpStream,
    fw_server: &str,
) -> Result<(), Box<dyn Error>> {
    info!("[Command] Query Configuration.");

    let all_config_history = read_config_from_pg(fw_server).await;

    match all_config_history {
        Ok(datas) => {
            let last_config_history = get_latest_config(&datas);
            match last_config_history {
                Some(config) => {
                    send_config_pkg(config, socket).await?;
                }
                None => {
                    send_failed_package(socket, ErrorCode::NoFirmwareFound as u8).await?;
                }
            }
        }
        Err(e) => {
            error!("Error:{}", e);
        }
    }

    Ok(())
}

/// 处理固件查询请求
async fn process_fw_query_request(
    _request: &[u8],
    socket: &mut TcpStream,
    code: i32,
    fw_data_all: &Vec<FirmwareData>,
) -> Result<(), Box<dyn Error>> {
    info!("[Command] Query Firmware Info.");
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
        send_failed_package(socket, ErrorCode::NoFirmwareFound as u8).await?;
    }

    Ok(())
}

/// 处理固件下载请求
async fn process_fw_download_request(
    request: &[u8],
    socket: &mut TcpStream,
    _code: i32,
    fw_data_all: &Vec<FirmwareData>,
) -> Result<(), Box<dyn Error>> {
    info!("[Command] Download Firmware.");

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
                info!(
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
                send_failed_package(socket, ErrorCode::FirmwareReadError as u8).await?;
            }
        }
    } else {
        error!("No firmware found!");
        send_failed_package(socket, ErrorCode::NoFirmwareFound as u8).await?;
    }

    Ok(())
}

/// 处理固件结束请求
async fn process_fw_end_request(
    request: &[u8],
    _socket: &mut TcpStream,
    _code: i32,
    _fw_data_all: &Vec<FirmwareData>,
) -> Result<(), Box<dyn Error>> {
    info!("[Command] Download Firmware Over.");

    let _version: FirmwareVersion = FirmwareVersion {
        m: request[7] as i32,
        n: request[8] as i32,
        l: request[9] as i32,
    };

    // 设备信息
    let device_id = (request[10] as i64) << 56
        | (request[11] as i64) << 48
        | (request[12] as i64) << 40
        | (request[13] as i64) << 32
        | (request[14] as i64) << 24
        | (request[15] as i64) << 16
        | (request[16] as i64) << 8
        | request[17] as i64;

    let sn = (request[18] as i32) << 24
        | (request[19] as i32) << 16
        | (request[20] as i32) << 8
        | request[21] as i32;

    let success = request[22] == 0xA1;

    let new_history = NewUpgradeHistory {
        sn,
        device_id,
        fwcode: _code,
        version_m: request[7] as i32,
        version_n: request[8] as i32,
        version_l: request[9] as i32,
        success,
    };

    // 插入数据库
    // let mut conn = pool.get()?;
    // match UpgradeHistory::create(new_history.clone(), &mut conn) {
    //     Ok(_) => {
    //         info!("Add upgrade histroy success. {}", new_history);
    //     }
    //     Err(e) => {
    //         error!("Add upgrade histroy failed. {}", e);
    //     }
    // }

    Ok(())
}
