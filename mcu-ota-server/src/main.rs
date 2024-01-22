use clap::Parser;
use firmware::from_pg::read_all_fw_from_pg;
use log::{error, info};
use mcu_ota_server::{args::Cli, process_pg::handle_client};

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
    env::set_var("RUST_APP_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    // parameters
    let fw_server = env::var("FW_SERVER").unwrap_or_else(|_| (cli.fw_server.clone()).to_string());
    let port = env::var("PORT").unwrap_or_else(|_| (cli.port.clone() as u32).to_string());
    let fw_db = Arc::new(env::var("FW_DB").unwrap_or_else(|_| cli.fw_db.clone()));

    // Create a listener
    let server = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&server).await?;
    info!("Server listening on {}", &server);

    loop {
        // 接受一个新的客户端连接
        let (socket, _) = listener.accept().await?;

        // 复制原子变量
        // let _fw_data_all = Arc::clone(&fw_data_all);
        let _fw_db = Arc::clone(&fw_db);

        // 刷新固件列表
        let _fw_data_all = Arc::new(read_all_fw_from_pg(&fw_server.clone()).await.unwrap());

        // 使用tokio的spawn函数，在独立的任务中处理每个客户端连接
        tokio::spawn(async move {
            if let Err(error) = handle_client(socket, &_fw_data_all, _fw_db).await {
                error!("Error handling client: {}", error);
            }
        });
    }
}
