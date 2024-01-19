use crate::{
    models::basic::{CrudOperations, HasId},
    schema::firmware_data,
    DbError,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{AsChangeset, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde_derive::{Deserialize, Serialize};

use super::basic::random_i32;

#[derive(Deserialize, Serialize, Queryable, Debug, AsChangeset, PartialEq, Default)]
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

#[derive(Debug, Insertable, Deserialize, Serialize, Default, PartialEq, Clone)]
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
