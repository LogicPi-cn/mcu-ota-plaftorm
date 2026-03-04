use crate::controls::{config_history, firmware_data, upgrade_history, user};
use actix_web::{get, web, Scope, HttpResponse, Responder};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "OTA Backend is running"
    }))
}

fn firmware_data_scope(path: &str) -> Scope {
    web::scope(path)
        .service(firmware_data::index)
        .service(firmware_data::create)
        .service(firmware_data::find)
        .service(firmware_data::update)
        .service(firmware_data::delete)
}

fn upgrade_history_scope(path: &str) -> Scope {
    web::scope(path)
        .service(upgrade_history::index)
        .service(upgrade_history::create)
        .service(upgrade_history::find)
        .service(upgrade_history::update)
        .service(upgrade_history::delete)
}

fn config_history_scope(path: &str) -> Scope {
    web::scope(path)
        .service(config_history::index)
        .service(config_history::create)
        .service(config_history::find)
        .service(config_history::update)
        .service(config_history::delete)
}

fn auth_scope(path: &str) -> Scope {
    web::scope(path)
        .service(user::register)
        .service(user::login)
        .service(user::logout)
        .service(user::get_me)
}

pub fn apis() -> Scope {
    web::scope("")
        .service(health_checker_handler)
        .service(auth_scope("/auth"))
        .service(upgrade_history_scope("/history"))
        .service(firmware_data_scope("/firmware"))
        .service(config_history_scope("/config"))
}
