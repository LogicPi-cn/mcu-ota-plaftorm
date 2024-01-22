use std::fmt;

use crate::{
    models::basic::{CrudOperations, HasId},
    schema::firmware_data,
    DbError,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{AsChangeset, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde_derive::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Queryable, Debug, AsChangeset, PartialEq, Default, Eq, Clone)]
#[diesel(table_name = firmware_data)]
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

#[derive(Debug, Insertable, Deserialize, Serialize, Default, PartialEq, Clone, Eq)]
#[diesel(table_name = firmware_data)]
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

#[derive(Debug, Deserialize, AsChangeset, Serialize, Default, Clone)]
#[diesel(table_name = firmware_data )]
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

impl CrudOperations<FirmwareData, NewFirmwareData, UpdateFirmwareData> for FirmwareData {
    fn all(conn: &mut PgConnection) -> Result<Vec<FirmwareData>, DbError> {
        let items = firmware_data::table.load::<Self>(conn)?;
        Ok(items)
    }

    fn find(target_id: i32, conn: &mut PgConnection) -> Result<FirmwareData, DbError> {
        let result = firmware_data::table
            .find(target_id)
            .first::<FirmwareData>(conn)?;
        Ok(result)
    }

    fn create(data: NewFirmwareData, conn: &mut PgConnection) -> Result<FirmwareData, DbError> {
        let result = diesel::insert_into(firmware_data::table)
            .values(&data)
            .get_result(conn)
            .expect("Error on Create");
        Ok(result)
    }

    fn update(
        id: i32,
        data: UpdateFirmwareData,
        conn: &mut PgConnection,
    ) -> Result<FirmwareData, DbError> {
        let result = diesel::update(firmware_data::table.find(id))
            .set(&data)
            .get_result(conn)
            .expect("Error on Update");
        Ok(result)
    }

    fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
        let num_deleted = diesel::delete(firmware_data::table.find(id))
            .execute(conn)
            .expect("Error on Delete");
        Ok(num_deleted)
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
