use crate::{
    db::Database,
    middleware::jwt_auth,
    models::user::{LoginUserSchema, RegisterUserSchema, TokenClaims, User},
};

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

#[post("/register")]
async fn register(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let pool = &data.pool;

    let exists = User::find_by_email(&body.email.to_string().to_lowercase(), pool).await;

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
    let hashed_password = match Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
    {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Failed to hash password"
            })));
        }
    };

    let new_user = crate::models::user::NewUser {
        username: body.username.to_string(),
        email: body.email.to_string().to_lowercase(),
        password: hashed_password,
    };

    let user = match User::create(new_user, pool).await {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Failed to create user"
            })));
        }
    };

    let user_response = json!({
        "status": "success",
        "data": {
            "user": user,
        }
    });

    Ok(HttpResponse::Ok().json(user_response))
}

#[post("/login")]
async fn login(
    body: web::Json<LoginUserSchema>,
    data: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let pool = &data.pool;

    let user = match User::find_by_email(&body.email.to_string().to_lowercase(), pool).await {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "User with that email doesn't exist" })));
        }
    };

    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Invalid password hash format"
            })));
        }
    };

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

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-secret".to_string());

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ) {
        Ok(token) => token,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Failed to generate token"
            })));
        }
    };

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

#[get("/logout")]
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
    let user_id = match ext.get::<uuid::Uuid>() {
        Some(id) => id,
        None => return HttpResponse::InternalServerError().json(json!({
            "status": "fail",
            "message": "Failed to get user ID"
        })),
    };

    let pool = &data.pool;

    let user = match User::find_by_id(&user_id, pool).await {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Failed to get user"
            }));
        }
    };

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": &user
        })
    });

    HttpResponse::Ok().json(json_response)
}
