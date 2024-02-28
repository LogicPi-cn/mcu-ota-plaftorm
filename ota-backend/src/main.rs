use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use clap::Parser;
use dotenv::dotenv;

use ota_backend::LOGO;

use log::info;
use ota_backend::args::Cli;
use ota_database::db::Config;
use ota_database::{db::Database, routes::total::apis};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get Parameters
    let cli = Cli::parse();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    // print logo
    println!("{}", LOGO);
    let version = env!("CARGO_PKG_VERSION");
    println!("OTA Backend, Version: {}", version);

    let _fw_db = env::var("FW_DB").unwrap_or_else(|_| cli.fw_db.clone());
    let _port = env::var("PORT").unwrap_or_else(|_| (cli.port as u32).to_string());
    let _db = Database::new(&_fw_db);

    // Create a listener
    let server = format!("0.0.0.0:{}", _port);
    info!("Server listening on {}", &server);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(Database {
                pool: _db.pool.clone(),
                env: config.clone(),
            }))
            .service(apis())
    })
    .bind(server)?
    .run()
    .await
}
