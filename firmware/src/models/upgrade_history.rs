use crate::{
    models::basic::{CrudOperations, HasId},
    schema::upgrade_history,
    DbError,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{AsChangeset, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde_derive::{Deserialize, Serialize};

use super::basic::{random_i32, random_i64};

#[derive(Deserialize, Serialize, Queryable, Debug, AsChangeset, PartialEq, Default)]
#[diesel(table_name = upgrade_history)]
pub struct UpgradeHistory {
    pub id: i32,
    pub sn: i32,
    pub device_id: i64,
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

#[derive(Debug, Insertable, Deserialize, Serialize, Default, PartialEq, Clone)]
#[diesel(table_name = upgrade_history)]
pub struct NewUpgradeHistory {
    pub sn: i32,
    pub device_id: i64,
    pub fwcode: i32,
    pub version_m: i32,
    pub version_n: i32,
    pub version_l: i32,
    pub success: bool,
}

impl NewUpgradeHistory {
    pub fn random() -> Self {
        NewUpgradeHistory {
            sn: random_i32(),
            device_id: random_i64(),
            fwcode: random_i32(),
            version_m: random_i32(),
            version_n: random_i32(),
            version_l: random_i32(),
            success: true,
        }
    }
}

#[derive(Debug, Deserialize, AsChangeset, Serialize, Default, Clone)]
#[diesel(table_name = upgrade_history )]
pub struct UpdateUpgradeHistory {
    pub sn: i32,
    pub device_id: i64,
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
            sn: random_i32(),
            device_id: random_i64(),
            fwcode: random_i32(),
            version_m: random_i32(),
            version_n: random_i32(),
            version_l: random_i32(),
            success: true,
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

impl CrudOperations<UpgradeHistory, NewUpgradeHistory, UpdateUpgradeHistory> for UpgradeHistory {
    fn all(conn: &mut PgConnection) -> Result<Vec<UpgradeHistory>, DbError> {
        let items = upgrade_history::table.load::<Self>(conn)?;
        Ok(items)
    }

    fn find(target_id: i32, conn: &mut PgConnection) -> Result<UpgradeHistory, DbError> {
        let result = upgrade_history::table
            .find(target_id)
            .first::<UpgradeHistory>(conn)?;
        Ok(result)
    }

    fn create(data: NewUpgradeHistory, conn: &mut PgConnection) -> Result<UpgradeHistory, DbError> {
        let result = diesel::insert_into(upgrade_history::table)
            .values(&data)
            .get_result(conn)
            .expect("Error on Create");
        Ok(result)
    }

    fn update(
        id: i32,
        data: UpdateUpgradeHistory,
        conn: &mut PgConnection,
    ) -> Result<UpgradeHistory, DbError> {
        let result = diesel::update(upgrade_history::table.find(id))
            .set(&data)
            .get_result(conn)
            .expect("Error on Update");
        Ok(result)
    }

    fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
        let num_deleted = diesel::delete(upgrade_history::table.find(id))
            .execute(conn)
            .expect("Error on Delete");
        Ok(num_deleted)
    }
}
