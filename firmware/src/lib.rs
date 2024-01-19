use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use sqlx::{Pool, Postgres};

pub mod common;
pub mod config;
pub mod controls;
pub mod from_http;
pub mod models;
pub mod routes;
pub mod schema;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub db: Pool<Postgres>,
    // pub env: Config,
}
