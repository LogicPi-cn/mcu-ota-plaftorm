use core::fmt;
use std::collections::HashSet;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Serialize;

use crate::db::Database;
use crate::models::user::TokenClaims;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = match req.app_data::<web::Data<Database>>() {
            Some(d) => d,
            None => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Internal server error".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.split_at(7).1.to_string())
            });

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        // Create validation with specific algorithm and enable expiry validation
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.required_spec_claims = HashSet::from(["sub".to_string(), "exp".to_string(), "iat".to_string()]);

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret".to_string());

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &validation,
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid or expired token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        let user_id = match uuid::Uuid::parse_str(claims.sub.as_str()) {
            Ok(id) => id,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token format".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id);

        ready(Ok(JwtMiddleware { user_id }))
    }
}
