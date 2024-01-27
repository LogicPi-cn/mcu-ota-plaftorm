use clap::Parser;
use log::{error, info};
use ota_database::from_pg::read_all_fw_from_pg;
use ota_server::{args::Cli, process_pg::handle_client};

use std::sync::Arc;
use std::{env, error::Error};
use tokio::net::TcpListener;

/// LogicPi Logo
const LOGO: &str = r"
    __    ____   ______ ____ ______ ____   ____
   / /   / __ \ / ____//  _// ____// __ \ /  _/
  / /   / / / // / __  / / / /    / /_/ / / /  
 / /___/ /_/ // /_/ /_/ / / /___ / ____/_/ /   
/_____/\____/ \____//___/ \____//_/    /___/   
";

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

    loop {
        // 接受一个新的客户端连接
        let (socket, _) = listener.accept().await?;

        // 原子变量引用
        let fw_server_clone = Arc::clone(&fw_server);

        // 刷新固件列表
        let _fw_data_all = Arc::new(read_all_fw_from_pg(&fw_server_clone).await.unwrap());

        // 使用tokio的spawn函数，在独立的任务中处理每个客户端连接
        tokio::spawn(async move {
            if let Err(error) = handle_client(socket, &_fw_data_all, &fw_server_clone).await {
                error!("Error handling client: {}", error);
            }
        });
    }
}
