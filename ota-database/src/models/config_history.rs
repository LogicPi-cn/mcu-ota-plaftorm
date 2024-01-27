use std::fmt;

use crate::{
    db::DbError,
    models::basic::{CrudOperations, HasId},
    schema::config_history,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{AsChangeset, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde_derive::{Deserialize, Serialize};

use super::basic::random_i32;

#[derive(Deserialize, Serialize, Queryable, Debug, AsChangeset, PartialEq, Default)]
#[diesel(table_name = config_history)]
pub struct ConfigHistory {
    pub id: i32,
    pub group_id: i32,
    pub op_code: i32,
    pub sync_ts: NaiveDateTime,
    pub interval: i32,
    pub t_max: i32,
    pub t_min: i32,
    pub human: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl HasId for ConfigHistory {
    fn id(&self) -> i32 {
        self.id
    }
}

/// 格式化打印
impl fmt::Display for ConfigHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConfigHistory -> Group:{:08X}, OpCode:{:08X}, Interval:{}, Tmax:{}, Tmin:{}",
            self.group_id, self.op_code, self.interval, self.t_max, self.t_min,
        )
    }
}

#[derive(Debug, Insertable, Deserialize, Serialize, Default, PartialEq, Clone)]
#[diesel(table_name = config_history)]
pub struct NewConfigHistory {
    pub group_id: i32,
    pub op_code: i32,
    pub sync_ts: NaiveDateTime,
    pub interval: i32,
    pub t_max: i32,
    pub t_min: i32,
    pub human: bool,
}

impl NewConfigHistory {
    pub fn random() -> Self {
        NewConfigHistory {
            group_id: random_i32(),
            op_code: random_i32(),
            sync_ts: Utc::now().naive_utc(),
            interval: random_i32(),
            t_max: random_i32(),
            t_min: random_i32(),
            human: false,
        }
    }
}

/// 格式化打印
impl fmt::Display for NewConfigHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConfigHistory -> Group:{:08X}, OpCode:{:08X}, Interval:{}, Tmax:{}, Tmin:{}",
            self.group_id, self.op_code, self.interval, self.t_max, self.t_min,
        )
    }
}

#[derive(Debug, Deserialize, AsChangeset, Serialize, Default, Clone)]
#[diesel(table_name = config_history )]
pub struct UpdateConfigHistory {
    pub group_id: i32,
    pub op_code: i32,
    pub sync_ts: NaiveDateTime,
    pub interval: i32,
    pub t_max: i32,
    pub t_min: i32,
    pub human: bool,
    pub updated_at: Option<NaiveDateTime>,
}

impl UpdateConfigHistory {
    pub fn random() -> Self {
        UpdateConfigHistory {
            group_id: random_i32(),
            op_code: random_i32(),
            sync_ts: Utc::now().naive_utc(),
            interval: random_i32(),
            t_max: random_i32(),
            t_min: random_i32(),
            human: false,
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

/// 格式化打印
impl fmt::Display for UpdateConfigHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConfigHistory -> Group:{:08X}, OpCode:{:08X}, Interval:{}, Tmax:{}, Tmin:{}",
            self.group_id, self.op_code, self.interval, self.t_max, self.t_min,
        )
    }
}

impl CrudOperations<ConfigHistory, NewConfigHistory, UpdateConfigHistory> for ConfigHistory {
    fn all(conn: &mut PgConnection) -> Result<Vec<ConfigHistory>, DbError> {
        let items = config_history::table.load::<Self>(conn)?;
        Ok(items)
    }

    fn find(target_id: i32, conn: &mut PgConnection) -> Result<ConfigHistory, DbError> {
        let result = config_history::table
            .find(target_id)
            .first::<ConfigHistory>(conn)?;
        Ok(result)
    }

    fn create(data: NewConfigHistory, conn: &mut PgConnection) -> Result<ConfigHistory, DbError> {
        let result = diesel::insert_into(config_history::table)
            .values(&data)
            .get_result(conn)
            .expect("Error on Create");
        Ok(result)
    }

    fn update(
        id: i32,
        data: UpdateConfigHistory,
        conn: &mut PgConnection,
    ) -> Result<ConfigHistory, DbError> {
        let result = diesel::update(config_history::table.find(id))
            .set(&data)
            .get_result(conn)
            .expect("Error on Update");
        Ok(result)
    }

    fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
        let num_deleted = diesel::delete(config_history::table.find(id))
            .execute(conn)
            .expect("Error on Delete");
        Ok(num_deleted)
    }
}
