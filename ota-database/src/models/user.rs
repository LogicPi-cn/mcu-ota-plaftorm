use std::fmt;

use crate::db::DatabaseError;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use super::basic::random_string;

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RegisterUserSchema {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub password: String,
    pub email: String,
}

/// 格式化打印
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User -> Username:{}, Email:{}",
            self.username, self.email
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl NewUser {
    pub fn random() -> Self {
        NewUser {
            username: random_string(10),
            password: random_string(10),
            email: random_string(10),
        }
    }
}

/// 格式化打印
impl fmt::Display for NewUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User -> Username:{}, Email:{}",
            self.username, self.email
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct UpdateUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub verified: bool,
    pub updated_at: Option<NaiveDateTime>,
}

impl UpdateUser {
    pub fn random() -> Self {
        UpdateUser {
            username: random_string(10),
            password: random_string(10),
            email: random_string(10),
            verified: false,
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

/// 格式化打印
impl fmt::Display for UpdateUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User -> Username:{}, Email:{}",
            self.username, self.email
        )
    }
}

impl User {
    pub async fn all(pool: &PgPool) -> Result<Vec<User>, DatabaseError> {
        let items = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        Ok(items)
    }

    pub async fn find_by_id(target_id: &uuid::Uuid, pool: &PgPool) -> Result<User, DatabaseError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(target_id)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    pub async fn find_by_email(target_email: &str, pool: &PgPool) -> Result<User, DatabaseError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(target_email)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    pub async fn create(data: NewUser, pool: &PgPool) -> Result<User, DatabaseError> {
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, password, email)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(data.username)
        .bind(data.password)
        .bind(data.email)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    pub async fn update_by_email(
        target_email: &str,
        updated_data: UpdateUser,
        pool: &PgPool,
    ) -> Result<User, DatabaseError> {
        let result = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET username = $1, password = $2, email = $3, verified = $4, updated_at = $5
            WHERE email = $6
            RETURNING *
            "#
        )
        .bind(updated_data.username)
        .bind(updated_data.password)
        .bind(updated_data.email)
        .bind(updated_data.verified)
        .bind(updated_data.updated_at.unwrap_or_else(|| Utc::now().naive_utc()))
        .bind(target_email)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    pub async fn delete_by_email(target_email: &str, pool: &PgPool) -> Result<u64, DatabaseError> {
        let result = sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(target_email)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
