use crate::controls::{config_history, firmware_data, upgrade_history};
use actix_web::{web, Scope};

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

pub fn apis() -> Scope {
    web::scope("")
        .service(upgrade_history_scope("/history"))
        .service(firmware_data_scope("/firmware"))
        .service(config_history_scope("/config"))
}
