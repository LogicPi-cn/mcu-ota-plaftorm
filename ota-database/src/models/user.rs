use std::fmt;

use crate::{
    db::DbError,
    models::basic::{CrudOperations, HasId},
    schema::users,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{AsChangeset, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde_derive::{Deserialize, Serialize};

use super::basic::random_string;

#[derive(Deserialize, Serialize, Queryable, Debug, AsChangeset, PartialEq, Default)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl HasId for User {
    fn id(&self) -> i32 {
        self.id
    }
}

/// 格式化打印
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User -> FirstName:{}, LastName:{}, Email:{}, Phone:{}",
            self.first_name, self.last_name, self.email, self.phone
        )
    }
}

#[derive(Debug, Insertable, Deserialize, Serialize, Default, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
}

impl NewUser {
    pub fn random() -> Self {
        NewUser {
            first_name: random_string(10),
            last_name: random_string(10),
            email: random_string(10),
            phone: random_string(10),
        }
    }
}

/// 格式化打印
impl fmt::Display for NewUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User -> FirstName:{}, LastName:{}, Email:{}, Phone:{}",
            self.first_name, self.last_name, self.email, self.phone
        )
    }
}

#[derive(Debug, Deserialize, AsChangeset, Serialize, Default, Clone)]
#[diesel(table_name = users )]
pub struct UpdateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub updated_at: Option<NaiveDateTime>,
}

impl UpdateUser {
    pub fn random() -> Self {
        UpdateUser {
            first_name: random_string(10),
            last_name: random_string(10),
            email: random_string(10),
            phone: random_string(10),
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

/// 格式化打印
impl fmt::Display for UpdateUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User -> FirstName:{}, LastName:{}, Email:{}, Phone:{}",
            self.first_name, self.last_name, self.email, self.phone
        )
    }
}

impl CrudOperations<User, NewUser, UpdateUser> for User {
    fn all(conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
        let items = users::table.load::<Self>(conn)?;
        Ok(items)
    }

    fn find(target_id: i32, conn: &mut PgConnection) -> Result<User, DbError> {
        let result = users::table.find(target_id).first::<User>(conn)?;
        Ok(result)
    }

    fn create(data: NewUser, conn: &mut PgConnection) -> Result<User, DbError> {
        let result = diesel::insert_into(users::table)
            .values(&data)
            .get_result(conn)
            .expect("Error on Create");
        Ok(result)
    }

    fn update(id: i32, data: UpdateUser, conn: &mut PgConnection) -> Result<User, DbError> {
        let result = diesel::update(users::table.find(id))
            .set(&data)
            .get_result(conn)
            .expect("Error on Update");
        Ok(result)
    }

    fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
        let num_deleted = diesel::delete(users::table.find(id))
            .execute(conn)
            .expect("Error on Delete");
        Ok(num_deleted)
    }
}
