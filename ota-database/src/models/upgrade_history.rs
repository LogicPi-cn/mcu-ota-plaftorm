use std::fmt;

use crate::db::DatabaseError;
use crate::models::basic::{CrudOperations, HasId};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use super::basic::{random_i32, random_string};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, FromRow)]
pub struct UpgradeHistory {
    pub id: i32,
    pub sn: String,
    pub device_id: String,
    pub fwcode: i32,
    pub version_m: i32,
    pub version_n: i32,
    pub version_l: i32,
    pub success: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl HasId for UpgradeHistory {
    fn id(&self) -> i32 {
        self.id
    }
}

/// 格式化打印
impl fmt::Display for UpgradeHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UpgradeHistory -> sn:{}, device_id:{}, Code:{:04X}, Version:{}.{}.{}, Status:{}",
            self.sn,
            self.device_id,
            self.fwcode,
            self.version_m,
            self.version_n,
            self.version_l,
            self.success
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone)]
pub struct NewUpgradeHistory {
    pub sn: String,
    pub device_id: String,
    pub fwcode: i32,
    pub version_m: i32,
    pub version_n: i32,
    pub version_l: i32,
    pub success: bool,
}

impl NewUpgradeHistory {
    pub fn random() -> Self {
        NewUpgradeHistory {
            sn: random_string(4),
            device_id: random_string(8),
            fwcode: random_i32(),
            version_m: random_i32(),
            version_n: random_i32(),
            version_l: random_i32(),
            success: true,
        }
    }
}

/// 格式化打印
impl fmt::Display for NewUpgradeHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UpgradeHistory -> sn:{}, device_id:{}, Code:{:04X}, Version:{}.{}.{}, Status:{}",
            self.sn,
            self.device_id,
            self.fwcode,
            self.version_m,
            self.version_n,
            self.version_l,
            self.success
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct UpdateUpgradeHistory {
    pub sn: String,
    pub device_id: String,
    pub fwcode: i32,
    pub version_m: i32,
    pub version_n: i32,
    pub version_l: i32,
    pub success: bool,
    pub updated_at: Option<NaiveDateTime>,
}

impl UpdateUpgradeHistory {
    pub fn random() -> Self {
        UpdateUpgradeHistory {
            sn: random_string(4),
            device_id: random_string(8),
            fwcode: random_i32(),
            version_m: random_i32(),
            version_n: random_i32(),
            version_l: random_i32(),
            success: true,
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

/// 格式化打印
impl fmt::Display for UpdateUpgradeHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UpgradeHistory -> sn:{}, device_id:{}, Code:{:04X}, Version:{}.{}.{}, Status:{}",
            self.sn,
            self.device_id,
            self.fwcode,
            self.version_m,
            self.version_n,
            self.version_l,
            self.success
        )
    }
}

#[async_trait::async_trait]
impl CrudOperations<UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory> for UpgradeHistory {
    async fn all(pool: &PgPool) -> Result<Vec<UpgradeHistory>, DatabaseError> {
        let items = sqlx::query_as::<_, UpgradeHistory>("SELECT * FROM upgrade_history")
            .fetch_all(pool)
            .await?;
        Ok(items)
    }

    async fn find(target_id: i32, pool: &PgPool) -> Result<UpgradeHistory, DatabaseError> {
        let result = sqlx::query_as::<_, UpgradeHistory>("SELECT * FROM upgrade_history WHERE id = $1")
            .bind(target_id)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    async fn create(data: NewUpgradeHistory, pool: &PgPool) -> Result<UpgradeHistory, DatabaseError> {
        let result = sqlx::query_as::<_, UpgradeHistory>(
            r#"
            INSERT INTO upgrade_history (sn, device_id, fwcode, version_m, version_n, version_l, success)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(data.sn)
        .bind(data.device_id)
        .bind(data.fwcode)
        .bind(data.version_m)
        .bind(data.version_n)
        .bind(data.version_l)
        .bind(data.success)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn update(
        id: i32,
        data: UpdateUpgradeHistory,
        pool: &PgPool,
    ) -> Result<UpgradeHistory, DatabaseError> {
        let result = sqlx::query_as::<_, UpgradeHistory>(
            r#"
            UPDATE upgrade_history
            SET sn = $1, device_id = $2, fwcode = $3, version_m = $4, version_n = $5, version_l = $6, success = $7, updated_at = $8
            WHERE id = $9
            RETURNING *
            "#
        )
        .bind(data.sn)
        .bind(data.device_id)
        .bind(data.fwcode)
        .bind(data.version_m)
        .bind(data.version_n)
        .bind(data.version_l)
        .bind(data.success)
        .bind(data.updated_at.unwrap_or_else(|| Utc::now().naive_utc()))
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn delete(id: i32, pool: &PgPool) -> Result<u64, DatabaseError> {
        let result = sqlx::query("DELETE FROM upgrade_history WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
