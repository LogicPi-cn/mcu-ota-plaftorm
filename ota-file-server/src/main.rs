use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use firmware::{common::FirmwareInfo, from_disk::list_all_fw};
use log::info;
use ota_file_server::args::Cli;
use std::{env, io};

/// LogicPi Logo
const LOGO: &str = r"
    __    ____   ______ ____ ______ ____   ____
   / /   / __ \ / ____//  _// ____// __ \ /  _/
  / /   / / / // / __  / / / /    / /_/ / / /  
 / /___/ /_/ // /_/ /_/ / / /___ / ____/_/ /   
/_____/\____/ \____//___/ \____//_/    /___/   
";

async fn index() -> impl Responder {
    "Welcome to the file server!"
}

async fn list_files() -> io::Result<HttpResponse> {
    let cli = Cli::parse();
    let dir_path = cli.fw_path;
    let all_fw_files: Vec<FirmwareInfo> = list_all_fw(&dir_path);
    Ok(HttpResponse::Ok().json(all_fw_files))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get Parameters
    let cli = Cli::parse();

    // print logo
    println!("{}", LOGO);
    let version = env!("CARGO_PKG_VERSION");
    println!("OTA File Server, Version: {}", version);

    // set log level
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let _fw_path = env::var("FW_PATH").unwrap_or_else(|_| cli.fw_path.clone());
    let _port = env::var("PORT").unwrap_or_else(|_| (cli.port as u32).to_string());

    // Create a listener
    let server = format!("0.0.0.0:{}", _port);
    info!("Server listening on {}", &server);
    info!("Firmware Storage Dir: {}", &_fw_path);

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/list", web::get().to(list_files))
            .service(Files::new("/download", format!("{}", _fw_path)))
    })
    .bind(server)?
    .run()
    .await
}
