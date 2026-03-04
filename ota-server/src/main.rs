use clap::Parser;
use log::{error, info};
use ota_server::{args::Cli, process_pg::handle_client, LOGO};

use std::sync::Arc;
use std::{env, error::Error};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get Parameters
    let cli = Cli::parse();

    // print logo
    println!("{}", LOGO);
    let version = env!("CARGO_PKG_VERSION");
    println!("OTA Server for IoT Devices, Version: {}", version);

    // set log level
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    // parameters
    let fw_server =
        Arc::new(env::var("FW_SERVER").unwrap_or_else(|_| (cli.fw_server.clone()).to_string()));
    let port = env::var("PORT").unwrap_or_else(|_| (cli.port.clone() as u32).to_string());

    // Create a listener
    let server = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&server).await?;
    info!("Server listening on {}", &server);

    // Create an Arc Mutex to hold firmware data
    let fw_data_all = Arc::new(tokio::sync::Mutex::new(Vec::new()));

    // Spawn the background refresh task
    let fw_server_bg = Arc::clone(&fw_server);
    let fw_data_bg = Arc::clone(&fw_data_all);
    tokio::spawn(async move {
        ota_database::from_pg::refresh_firmware_data(&fw_server_bg, fw_data_bg).await;
    });

    loop {
        // 接受一个新的客户端连接
        let (socket, _) = listener.accept().await?;

        // 原子变量引用
        let fw_server_clone = Arc::clone(&fw_server);
        let fw_data_clone = Arc::clone(&fw_data_all);

        // 使用tokio的spawn函数，在独立的任务中处理每个客户端连接
        tokio::spawn(async move {
            if let Err(error) = handle_client(socket, fw_data_clone, &fw_server_clone).await {
                error!("Error handling client: {}", error);
            }
        });
    }
}
