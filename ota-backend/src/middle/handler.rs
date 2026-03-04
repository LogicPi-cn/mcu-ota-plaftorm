use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::Row;

use crate::middle::{
    app_state::AppState,
    jwt_auth,
    model::{LoginUserSchema, RegisterUserSchema, TokenClaims},
};

use super::{model::User, response::FilteredUser};

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres, and SQLX";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let exists: bool = match sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned())
        .fetch_one(&data.db)
        .await
    {
        Ok(row) => row.get(0),
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Database error"
            }));
        }
    };

    if exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = match Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
    {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Failed to hash password"
            }));
        }
    };

    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.email.to_string().to_lowercase(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": filter_user_record(&user)
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "fail","message": "Failed to create user"}));
        }
    }
}
#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = match sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email)
        .fetch_optional(&data.db)
        .await
    {
        Ok(result) => result,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Database error"
            }));
        }
    };

    let is_valid = query_result.to_owned().and_then(|user| {
        let parsed_hash = PasswordHash::new(&user.password).ok()?;
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .ok()?;
        Some(user)
    }).is_some();

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let user = query_result.unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    ) {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "fail",
                "message": "Failed to generate token"
            }));
        }
    };

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/auth/logout")]
async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
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

    let user = match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await
    {
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
            "user": filter_user_record(&user)
        })
    });

    HttpResponse::Ok().json(json_response)
}

fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        photo: user.photo.to_owned(),
        role: user.role.to_owned(),
        verified: user.verified,
        createdAt: user.created_at.unwrap_or_default(),
        updatedAt: user.updated_at.unwrap_or_default(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler)
        .service(logout_handler)
        .service(get_me_handler);

    conf.service(scope);
}
