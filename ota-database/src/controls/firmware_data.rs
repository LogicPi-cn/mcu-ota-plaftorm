use crate::{
    db::Database,
    models::{
        basic::CrudOperations,
        firmware_data::{FirmwareData, NewFirmwareData, UpdateFirmwareData},
    },
};

use actix_web::{delete, get, patch, post, web, Error, HttpResponse};

#[get("")]
pub async fn index(
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let items: Vec<FirmwareData> = <FirmwareData as CrudOperations<FirmwareData, NewFirmwareData, UpdateFirmwareData>>::all(&db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("")]
pub async fn create(
    db: web::Data<Database>,
    payload: web::Json<NewFirmwareData>,
) -> Result<HttpResponse, Error> {
    let item: FirmwareData = <FirmwareData as CrudOperations<FirmwareData, NewFirmwareData, UpdateFirmwareData>>::create(payload.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[get("/{id}")]
pub async fn find(
    id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let item: FirmwareData = <FirmwareData as CrudOperations<FirmwareData, NewFirmwareData, UpdateFirmwareData>>::find(id.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[patch("/{id}")]
pub async fn update(
    id: web::Path<i32>,
    payload: web::Json<UpdateFirmwareData>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let item: FirmwareData = <FirmwareData as CrudOperations<FirmwareData, NewFirmwareData, UpdateFirmwareData>>::update(id.into_inner(), payload.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[delete("/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let result: u64 = <FirmwareData as CrudOperations<FirmwareData, NewFirmwareData, UpdateFirmwareData>>::delete(id.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
