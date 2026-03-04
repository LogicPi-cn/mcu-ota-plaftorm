use crate::{
    db::Database,
    models::{
        basic::CrudOperations,
        config_history::{ConfigHistory, NewConfigHistory, UpdateConfigHistory},
    },
};

use actix_web::{delete, get, patch, post, web, Error, HttpResponse};

#[get("")]
pub async fn index(
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let items: Vec<ConfigHistory> = <ConfigHistory as CrudOperations<ConfigHistory, NewConfigHistory, UpdateConfigHistory>>::all(&db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(items))
}

#[post("")]
pub async fn create(
    db: web::Data<Database>,
    payload: web::Json<NewConfigHistory>,
) -> Result<HttpResponse, Error> {
    let item: ConfigHistory = <ConfigHistory as CrudOperations<ConfigHistory, NewConfigHistory, UpdateConfigHistory>>::create(payload.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[get("/{id}")]
pub async fn find(
    id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let item: ConfigHistory = <ConfigHistory as CrudOperations<ConfigHistory, NewConfigHistory, UpdateConfigHistory>>::find(id.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[patch("/{id}")]
pub async fn update(
    id: web::Path<i32>,
    payload: web::Json<UpdateConfigHistory>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let item: ConfigHistory = <ConfigHistory as CrudOperations<ConfigHistory, NewConfigHistory, UpdateConfigHistory>>::update(id.into_inner(), payload.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[delete("/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let result: u64 = <ConfigHistory as CrudOperations<ConfigHistory, NewConfigHistory, UpdateConfigHistory>>::delete(id.into_inner(), &db.pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
