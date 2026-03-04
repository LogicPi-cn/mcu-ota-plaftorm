use crate::{
    db::Database,
    models::{
        basic::CrudOperations,
        upgrade_history::{NewUpgradeHistory, UpdateUpgradeHistory, UpgradeHistory},
    },
};

use actix_web::{delete, get, patch, post, web, Error, HttpResponse};

#[get("")]
pub async fn index(
    data: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let items: Vec<UpgradeHistory> = <UpgradeHistory as CrudOperations<UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory>>::all(&data.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("")]
pub async fn create(
    data: web::Data<Database>,
    payload: web::Json<NewUpgradeHistory>,
) -> Result<HttpResponse, Error> {
    let item: UpgradeHistory = <UpgradeHistory as CrudOperations<UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory>>::create(payload.into_inner(), &data.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[get("/{id}")]
pub async fn find(
    id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let item: UpgradeHistory = <UpgradeHistory as CrudOperations<UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory>>::find(id.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[patch("/{id}")]
pub async fn update(
    id: web::Path<i32>,
    payload: web::Json<UpdateUpgradeHistory>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let item: UpgradeHistory = <UpgradeHistory as CrudOperations<UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory>>::update(id.into_inner(), payload.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[delete("/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let result: u64 = <UpgradeHistory as CrudOperations<UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory>>::delete(id.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
