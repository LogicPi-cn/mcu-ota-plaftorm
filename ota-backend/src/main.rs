use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;

use log::info;
use ota_backend::args::Cli;
use ota_database::{db::Database, routes::total::apis};
use std::env;

/// LogicPi Logo
const LOGO: &str = r"
    __    ____   ______ ____ ______ ____   ____
   / /   / __ \ / ____//  _// ____// __ \ /  _/
  / /   / / / // / __  / / / /    / /_/ / / /  
 / /___/ /_/ // /_/ /_/ / / /___ / ____/_/ /   
/_____/\____/ \____//___/ \____//_/    /___/   
";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get Parameters
    let cli = Cli::parse();

    // print logo
    println!("{}", LOGO);
    let version = env!("CARGO_PKG_VERSION");
    println!("OTA File Server, Version: {}", version);

    // set log level
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_LOG");

    let _fw_path = env::var("FW_PATH").unwrap_or_else(|_| cli.fw_path.clone());
    let _fw_db = env::var("FW_DB").unwrap_or_else(|_| cli.fw_db.clone());
    let _port = env::var("PORT").unwrap_or_else(|_| (cli.port as u32).to_string());

    let db = Database::new(&_fw_db);

    // Create a listener
    let server = format!("0.0.0.0:{}", _port);
    info!("Server listening on {}", &server);
    info!("Firmware Storage Dir: {}", &_fw_path);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db.clone().pool))
            .service(apis())
    })
    .bind(server)?
    .run()
    .await
}
