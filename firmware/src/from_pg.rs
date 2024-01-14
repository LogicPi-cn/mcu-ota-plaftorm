use crate::common::{FirmwareData, FirmwareInfo};

pub struct Database {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl Database {
    pub async fn create_firmware_data(
        &self,
        firmware_data: &FirmwareData,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO firmware_data (info, data)
            VALUES ($1, $2)
            "#,
            serde_json::to_string(&firmware_data.info).unwrap(),
            firmware_data.data
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn read_firmware_data(&self, id: i32) -> Result<FirmwareData, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT info, data
            FROM firmware_data
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        let info: FirmwareInfo = serde_json::from_str(&row.info).unwrap();
        let data = row.data;

        Ok(FirmwareData { info, data })
    }

    pub async fn update_firmware_data(
        &self,
        id: i32,
        firmware_data: &FirmwareData,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE firmware_data
            SET info = $1, data = $2
            WHERE id = $3
            "#,
            serde_json::to_string(&firmware_data.info).unwrap(),
            firmware_data.data,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_firmware_data(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM firmware_data
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
