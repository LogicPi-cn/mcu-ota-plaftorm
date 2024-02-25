use crate::{
    db::Database,
    middleware::jwt_auth,
    models::user::{LoginUserSchema, NewUser, RegisterUserSchema, TokenClaims, UpdateUser, User},
};

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    delete, get, patch, post, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

// #[get("")]
// pub async fn index(data: web::Data<Database>) -> Result<HttpResponse, Error> {
//     let tweets = web::block(move || {
//         let mut conn = data.pool.get()?;
//         User::all(&mut conn)
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(tweets))
// }

#[post("")]
pub async fn create(
    data: web::Data<Database>,
    payload: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let data = web::block(move || {
        let mut conn = data.pool.get()?;
        User::create(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

// #[get("/{id}")]
// pub async fn find(id: web::Path<i32>, data: web::Data<Database>) -> Result<HttpResponse, Error> {
//     let data = web::block(move || {
//         let mut conn = data.pool.get()?;
//         User::find(id.into_inner(), &mut conn)
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(data))
// }

// #[patch("/{id}")]
// pub async fn update(
//     id: web::Path<i32>,
//     payload: web::Json<UpdateUser>,
//     data: web::Data<Database>,
// ) -> Result<HttpResponse, Error> {
//     let user = web::block(move || {
//         let mut conn = data.pool.get()?;
//         User::update(id.into_inner(), payload.into_inner(), &mut conn)
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(user))
// }

// #[delete("/{id}")]
// pub async fn delete(id: web::Path<i32>, data: web::Data<Database>) -> Result<HttpResponse, Error> {
//     let result = web::block(move || {
//         let mut conn = data.pool.get()?;
//         User::delete(id.into_inner(), &mut conn)
//     })
//     .await?
//     .map(|data| HttpResponse::Ok().json(data))
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(result)
// }

#[post("/auth/register")]
async fn register(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let mut conn = data.pool.get().expect("Couldn't get DB connection");

    let exists = User::find_by_email(&body.email.to_string().to_lowercase(), &mut conn);

    match exists {
        Ok(_) => {
            return Ok(HttpResponse::Conflict().json(json!({
                "status": "fail",
                "message": "User with that email already exists"
            })));
        }
        _ => {
            // continue with registration
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let new_user = NewUser {
        name: body.name.to_string(),
        email: body.email.to_string().to_lowercase(),
        password: hashed_password,
    };

    let user = User::create(new_user, &mut conn).expect("Error inserting new user");

    let user_response = json!({
        "status": "success",
        "data": {
            "user": user,
        }
    });

    Ok(HttpResponse::Ok().json(user_response))
}

#[post("/auth/login")]
async fn login(
    body: web::Json<LoginUserSchema>,
    data: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let mut conn = data.pool.get().expect("Couldn't get DB connection");

    let user = match User::find_by_email(&body.email.to_string().to_lowercase(), &mut conn) {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "User with that email doesn't exist" })));
        }
    };

    let parsed_hash = PasswordHash::new(&user.password).unwrap();

    match Argon2::default().verify_password(body.password.as_bytes(), &parsed_hash) {
        Ok(_) => {}
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "Invalid email or password"
            })));
        }
    };

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(json!({
        "status": "success",
        "token": token
    })))
}

#[get("/auth/logout")]
async fn logout(_: jwt_auth::JwtMiddleware) -> Result<HttpResponse, Error> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(json!({
        "status": "success"
    })))
}

#[get("/me")]
async fn get_me(
    req: HttpRequest,
    data: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    // let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
    //     .fetch_one(&data.db)
    //     .await
    //     .unwrap();

    let mut conn = data.pool.get().expect("Couldn't get DB connection");
    let user = User::find_by_id(&user_id, &mut conn).unwrap();

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": &user
        })
    });

    HttpResponse::Ok().json(json_response)
}
