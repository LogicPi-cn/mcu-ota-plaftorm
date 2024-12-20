use std::fmt;

use crate::{db::DbError, schema::users};
use chrono::{NaiveDateTime, Utc};
use diesel::{
    AsChangeset, ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde_derive::{Deserialize, Serialize};

use super::basic::random_string;

#[derive(Deserialize, Serialize, Queryable, Debug, AsChangeset, PartialEq, Default)]
#[diesel(table_name = users)]
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

#[derive(Debug, Insertable, Deserialize, Serialize, Default, PartialEq, Clone)]
#[diesel(table_name = users)]
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

#[derive(Debug, Deserialize, AsChangeset, Serialize, Default, Clone)]
#[diesel(table_name = users )]
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
    pub fn all(conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
        let items = users::table.load::<Self>(conn)?;
        Ok(items)
    }

    pub fn find_by_id(target_id: &uuid::Uuid, conn: &mut PgConnection) -> Result<User, DbError> {
        let result = users::table
            .filter(users::id.eq(target_id))
            .first::<User>(conn)?;
        Ok(result)
    }

    pub fn find_by_email(target_email: &str, conn: &mut PgConnection) -> Result<User, DbError> {
        let result = users::table
            .filter(users::email.eq(target_email))
            .first::<User>(conn)?;
        Ok(result)
    }

    pub fn create(data: NewUser, conn: &mut PgConnection) -> Result<User, DbError> {
        let result = diesel::insert_into(users::table)
            .values(&data)
            .get_result(conn)
            .expect("Error on Create");
        Ok(result)
    }

    pub fn update_by_email(
        target_email: &str,
        updated_data: UpdateUser,
        conn: &mut PgConnection,
    ) -> Result<User, DbError> {
        let updated_user = diesel::update(users::table.filter(users::email.eq(target_email)))
            .set(&updated_data)
            .get_result(conn)
            .expect("Error on Update");

        Ok(updated_user)
    }

    pub fn delete_by_email(target_email: &str, conn: &mut PgConnection) -> Result<usize, DbError> {
        let num_deleted = diesel::delete(users::table.filter(users::email.eq(target_email)))
            .execute(conn)
            .expect("Error on Delete");

        Ok(num_deleted)
    }
}
