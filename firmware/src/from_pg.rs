use crate::common::{FirmwareData, FirmwareInfo};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

pub struct Database {
    pub db: Pool<Postgres>,
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
        .execute(&self.db)
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
        .fetch_one(&self.db)
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
        .execute(&self.db)
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
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

#[post("/firmware")]
async fn create_firmware(
    db: web::Data<Database>,
    firmware_data: web::Json<FirmwareData>,
) -> impl Responder {
    let result = db.create_firmware_data(&firmware_data.0).await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/firmware/{id}")]
async fn read_firmware(db: web::Data<Database>, id: web::Path<i32>) -> impl Responder {
    let result = db.read_firmware_data(*id).await;
    match result {
        Ok(firmware_data) => HttpResponse::Ok().json(firmware_data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/firmware/{id}")]
async fn update_firmware(
    db: web::Data<Database>,
    id: web::Path<i32>,
    firmware_data: web::Json<FirmwareData>,
) -> impl Responder {
    let result = db.update_firmware_data(*id, &firmware_data.0).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/firmware/{id}")]
async fn delete_firmware(db: web::Data<Database>, id: web::Path<i32>) -> impl Responder {
    let result = db.delete_firmware_data(*id).await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
