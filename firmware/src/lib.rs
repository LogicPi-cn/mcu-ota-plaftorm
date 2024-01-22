use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub mod config;
pub mod controls;
pub mod from_pg;
pub mod models;
pub mod routes;
pub mod schema;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: DbPool,
}

impl Database {
    pub fn new(db_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: DbPool = r2d2::Pool::builder()
            .min_idle(Some(5)) // 最小空闲连接数
            .max_size(5) // 最大连接数
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }
}
