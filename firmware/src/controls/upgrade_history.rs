use crate::{
    models::{
        basic::CrudOperations,
        upgrade_history::{NewUpgradeHistory, UpdateUpgradeHistory, UpgradeHistory},
    },
    DbPool,
};

use actix_web::{delete, get, patch, post, web, Error, HttpResponse};

#[get("")]
pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let tweets = web::block(move || {
        let mut conn = pool.get()?;
        UpgradeHistory::all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweets))
}

#[post("")]
pub async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<NewUpgradeHistory>,
) -> Result<HttpResponse, Error> {
    let data = web::block(move || {
        let mut conn = pool.get()?;
        UpgradeHistory::create(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

#[get("/{id}")]
pub async fn find(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let data = web::block(move || {
        let mut conn = pool.get()?;
        UpgradeHistory::find(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

#[patch("/{id}")]
pub async fn update(
    id: web::Path<i32>,
    payload: web::Json<UpdateUpgradeHistory>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let tweet = web::block(move || {
        let mut conn = pool.get()?;
        UpgradeHistory::update(id.into_inner(), payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[delete("/{id}")]
pub async fn delete(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        UpgradeHistory::delete(id.into_inner(), &mut conn)
    })
    .await?
    .map(|data| HttpResponse::Ok().json(data))
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}
