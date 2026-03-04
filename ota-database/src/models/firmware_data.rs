use std::fmt;

use crate::db::DatabaseError;
use crate::models::basic::{CrudOperations, HasId};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use super::basic::random_i32;

/// 固件版本结构体
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FirmwareVersion {
    pub m: i32,
    pub n: i32,
    pub l: i32,
}

/// 固件信息
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct FirmwareInfo {
    pub code: i32,
    pub version: FirmwareVersion,
    pub size: i32,    // 以字节为单位
    pub path: String, // 文件路径
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Eq, Clone, FromRow)]
pub struct FirmwareData {
    pub id: i32,
    pub fwcode: i32,
    pub version_m: i32,
    pub version_n: i32,
    pub version_l: i32,
    pub fwsize: i32,
    pub fwdata: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl HasId for FirmwareData {
    fn id(&self) -> i32 {
        self.id
    }
}

/// 格式化打印
impl fmt::Display for FirmwareData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FwInfo -> Code:{:04X}, Version: {}.{}.{}, Size: {} bytes",
            self.fwcode, self.version_m, self.version_n, self.version_l, self.fwsize
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone, Eq)]
pub struct NewFirmwareData {
    pub fwcode: i32,
    pub version_m: i32,
    pub version_n: i32,
    pub version_l: i32,
    pub fwsize: i32,
    pub fwdata: Vec<u8>,
}
impl NewFirmwareData {
    pub fn random() -> Self {
        NewFirmwareData {
            fwcode: random_i32(),
            version_m: random_i32(),
            version_n: random_i32(),
            version_l: random_i32(),
            fwsize: random_i32(),
            fwdata: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
        }
    }
}

/// 格式化打印
impl fmt::Display for NewFirmwareData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FwInfo -> Code:{:04X}, Version: {}.{}.{}, Size: {} bytes",
            self.fwcode, self.version_m, self.version_n, self.version_l, self.fwsize
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct UpdateFirmwareData {
    pub fwcode: i32,
    pub version_m: i32,
    pub version_n: i32,
    pub version_l: i32,
    pub fwsize: i32,
    pub fwdata: Vec<u8>,
    pub updated_at: Option<NaiveDateTime>,
}

impl UpdateFirmwareData {
    pub fn random() -> Self {
        UpdateFirmwareData {
            fwcode: random_i32(),
            version_m: random_i32(),
            version_n: random_i32(),
            version_l: random_i32(),
            fwsize: random_i32(),
            fwdata: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

/// 格式化打印
impl fmt::Display for UpdateFirmwareData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FwInfo -> Code:{:04X}, Version: {}.{}.{}, Size: {} bytes",
            self.fwcode, self.version_m, self.version_n, self.version_l, self.fwsize
        )
    }
}

#[async_trait::async_trait]
impl CrudOperations<FirmwareData, NewFirmwareData, UpdateFirmwareData> for FirmwareData {
    async fn all(pool: &PgPool) -> Result<Vec<FirmwareData>, DatabaseError> {
        let items = sqlx::query_as::<_, FirmwareData>("SELECT * FROM firmware_data")
            .fetch_all(pool)
            .await?;
        Ok(items)
    }

    async fn find(target_id: i32, pool: &PgPool) -> Result<FirmwareData, DatabaseError> {
        let result = sqlx::query_as::<_, FirmwareData>("SELECT * FROM firmware_data WHERE id = $1")
            .bind(target_id)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    async fn create(data: NewFirmwareData, pool: &PgPool) -> Result<FirmwareData, DatabaseError> {
        let result = sqlx::query_as::<_, FirmwareData>(
            r#"
            INSERT INTO firmware_data (fwcode, version_m, version_n, version_l, fwsize, fwdata)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(data.fwcode)
        .bind(data.version_m)
        .bind(data.version_n)
        .bind(data.version_l)
        .bind(data.fwsize)
        .bind(data.fwdata)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn update(
        id: i32,
        data: UpdateFirmwareData,
        pool: &PgPool,
    ) -> Result<FirmwareData, DatabaseError> {
        let result = sqlx::query_as::<_, FirmwareData>(
            r#"
            UPDATE firmware_data
            SET fwcode = $1, version_m = $2, version_n = $3, version_l = $4, fwsize = $5, fwdata = $6, updated_at = $7
            WHERE id = $8
            RETURNING *
            "#
        )
        .bind(data.fwcode)
        .bind(data.version_m)
        .bind(data.version_n)
        .bind(data.version_l)
        .bind(data.fwsize)
        .bind(data.fwdata)
        .bind(data.updated_at.unwrap_or_else(|| Utc::now().naive_utc()))
        .bind(id)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn delete(id: i32, pool: &PgPool) -> Result<u64, DatabaseError> {
        let result = sqlx::query("DELETE FROM firmware_data WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}

/// 根据code查找最新版本的固件
pub fn find_latest_fw(all_fw_files: &[FirmwareData], code: i32) -> Option<FirmwareData> {
    let filtered_fw_files: Vec<&FirmwareData> =
        all_fw_files.iter().filter(|fw| fw.fwcode == code).collect();

    filtered_fw_files
        .into_iter()
        .max_by(|a, b| {
            let version_a_m = &a.version_m;
            let version_a_n = &a.version_n;
            let version_a_l = &a.version_l;
            let version_b_m = &b.version_m;
            let version_b_n = &b.version_n;
            let version_b_l = &b.version_l;
            (version_a_m, version_a_n, version_a_l).cmp(&(version_b_m, version_b_n, version_b_l))
        })
        .cloned()
}

/// 根据code和version查找具体固件
pub fn find_firmware(
    all_fw_files: &[FirmwareData],
    code: i32,
    version: FirmwareVersion,
) -> Option<FirmwareData> {
    all_fw_files
        .iter()
        .find(|firmware| {
            firmware.fwcode == code
                && firmware.version_m == version.m
                && firmware.version_n == version.n
                && firmware.version_l == version.l
        })
        .cloned()
}

/// 切片固件数据
pub fn slice_fw_data_from_vector(data: &[u8], index: usize, slice_size: usize) -> Option<Vec<u8>> {
    let start_position = index * slice_size;

    // 先判断start_position有没有越界
    if start_position >= data.len() {
        return None;
    }

    // 再判断end_position有没有越界
    let end_position = std::cmp::min(start_position + slice_size, data.len());

    if end_position <= start_position {
        return None;
    }

    Some(data[start_position..end_position].to_vec())
}
