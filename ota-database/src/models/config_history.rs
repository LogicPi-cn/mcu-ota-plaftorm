use std::fmt;

use crate::db::DatabaseError;
use crate::models::basic::{CrudOperations, HasId};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use super::basic::random_i32;

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, FromRow)]
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

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
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

#[async_trait::async_trait]
impl CrudOperations<ConfigHistory, NewConfigHistory, UpdateConfigHistory> for ConfigHistory {
    async fn all(pool: &PgPool) -> Result<Vec<ConfigHistory>, DatabaseError> {
        let items = sqlx::query_as::<_, ConfigHistory>("SELECT * FROM config_history")
            .fetch_all(pool)
            .await?;
        Ok(items)
    }

    async fn find(target_id: i32, pool: &PgPool) -> Result<ConfigHistory, DatabaseError> {
        let result =
            sqlx::query_as::<_, ConfigHistory>("SELECT * FROM config_history WHERE id = $1")
                .bind(target_id)
                .fetch_one(pool)
                .await?;
        Ok(result)
    }

    async fn create(data: NewConfigHistory, pool: &PgPool) -> Result<ConfigHistory, DatabaseError> {
        let result = sqlx::query_as::<_, ConfigHistory>(
            r#"
            INSERT INTO config_history (group_id, op_code, sync_ts, interval, t_max, t_min, human)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(data.group_id)
        .bind(data.op_code)
        .bind(data.sync_ts)
        .bind(data.interval)
        .bind(data.t_max)
        .bind(data.t_min)
        .bind(data.human)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn update(
        id: i32,
        data: UpdateConfigHistory,
        pool: &PgPool,
    ) -> Result<ConfigHistory, DatabaseError> {
        let result = sqlx::query_as::<_, ConfigHistory>(
            r#"
            UPDATE config_history
            SET group_id = $1, op_code = $2, sync_ts = $3, interval = $4, t_max = $5, t_min = $6, human = $7, updated_at = $8
            WHERE id = $9
            RETURNING *
            "#,
        )
        .bind(data.group_id)
        .bind(data.op_code)
        .bind(data.sync_ts)
        .bind(data.interval)
        .bind(data.t_max)
        .bind(data.t_min)
        .bind(data.human)
        .bind(data.updated_at.unwrap_or_else(|| Utc::now().naive_utc()))
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn delete(id: i32, pool: &PgPool) -> Result<u64, DatabaseError> {
        let result = sqlx::query("DELETE FROM config_history WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
