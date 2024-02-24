use crate::{
    db::Database,
    middleware::jwt_auth,
    models::{
        basic::CrudOperations,
        config_history::{ConfigHistory, NewConfigHistory, UpdateConfigHistory},
    },
};

use actix_web::{delete, get, patch, post, web, Error, HttpResponse};

#[get("")]
pub async fn index(
    db: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let tweets = web::block(move || {
        let mut conn = db.pool.get()?;
        ConfigHistory::all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweets))
}

#[post("")]
pub async fn create(
    db: web::Data<Database>,
    payload: web::Json<NewConfigHistory>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let data = web::block(move || {
        let mut conn = db.pool.get()?;
        ConfigHistory::create(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

#[get("/{id}")]
pub async fn find(
    id: web::Path<i32>,
    db: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let data = web::block(move || {
        let mut conn = db.pool.get()?;
        ConfigHistory::find(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

#[patch("/{id}")]
pub async fn update(
    id: web::Path<i32>,
    payload: web::Json<UpdateConfigHistory>,
    db: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let tweet = web::block(move || {
        let mut conn = db.pool.get()?;
        ConfigHistory::update(id.into_inner(), payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[delete("/{id}")]
pub async fn delete(
    id: web::Path<i32>,
    db: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = db.pool.get()?;
        ConfigHistory::delete(id.into_inner(), &mut conn)
    })
    .await?
    .map(|data| HttpResponse::Ok().json(data))
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}
