use crate::controls::{config_history, firmware_data, upgrade_history};
use actix_web::{web, Scope};

fn firmware_data(path: &str) -> Scope {
    let result = web::scope(path)
        .service(firmware_data::index)
        .service(firmware_data::create)
        .service(firmware_data::find)
        .service(firmware_data::update)
        .service(firmware_data::delete);
    return result;
}

fn upgrade_history(path: &str) -> Scope {
    let result = web::scope(path)
        .service(upgrade_history::index)
        .service(upgrade_history::create)
        .service(upgrade_history::find)
        .service(upgrade_history::update)
        .service(upgrade_history::delete);
    return result;
}

fn config_history(path: &str) -> Scope {
    let result = web::scope(path)
        .service(config_history::index)
        .service(config_history::create)
        .service(config_history::find)
        .service(config_history::update)
        .service(config_history::delete);
    return result;
}

pub fn apis() -> Scope {
    let service = web::scope("")
        .service(upgrade_history("/history"))
        .service(firmware_data("/firmware"))
        .service(config_history("/config"));
    return service;
}
