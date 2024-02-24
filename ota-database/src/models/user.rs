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
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
    pub role: Option<String>,
    pub photo: Option<String>,
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
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

/// 格式化打印
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User -> Name:{}, Email:{}", self.name, self.email)
    }
}

#[derive(Debug, Insertable, Deserialize, Serialize, Default, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn random() -> Self {
        NewUser {
            name: random_string(10),
            email: random_string(10),
            password: random_string(10),
        }
    }
}

/// 格式化打印
impl fmt::Display for NewUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User -> Name:{}, Email:{}", self.name, self.email)
    }
}

#[derive(Debug, Deserialize, AsChangeset, Serialize, Default, Clone)]
#[diesel(table_name = users )]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub updated_at: Option<NaiveDateTime>,
}

impl UpdateUser {
    pub fn random() -> Self {
        UpdateUser {
            name: random_string(10),
            email: random_string(10),
            phone: random_string(10),
            password: random_string(10),
            role: random_string(10),
            photo: random_string(10),
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
            "User -> Name:{}, Email:{}, Phone:{}",
            self.name, self.email, self.phone
        )
    }
}

impl User {
    // pub fn all(conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
    //     let items = users::table.load::<Self>(conn)?;
    //     Ok(items)
    // }

    // pub fn find(target_id: i32, conn: &mut PgConnection) -> Result<User, DbError> {
    //     let result = users::table.find(target_id).first::<User>(conn)?;
    //     Ok(result)
    // }

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

    // pub fn update(id: i32, data: UpdateUser, conn: &mut PgConnection) -> Result<User, DbError> {
    //     let result = diesel::update(users::table.find(id))
    //         .set(&data)
    //         .get_result(conn)
    //         .expect("Error on Update");
    //     Ok(result)
    // }

    // pub fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
    //     let num_deleted = diesel::delete(users::table.find(id))
    //         .execute(conn)
    //         .expect("Error on Delete");
    //     Ok(num_deleted)
    // }
}
