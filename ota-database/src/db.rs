use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to create database pool: {0}")]
    PoolCreationError(#[from] sqlx::Error),
    #[error("Environment variable {0} must be set")]
    MissingEnvVar(&'static str),
    #[error("Invalid environment variable: {0}")]
    InvalidEnvVar(&'static str),
}

pub type DbResult<T> = Result<T, DatabaseError>;
pub type DbPool = Pool<Postgres>;

/// Database connection pool wrapper
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: DbPool,
}

impl Database {
    pub async fn new(db_url: &str) -> DbResult<Self> {
        let pool = PgPoolOptions::new()
            .min_connections(5)
            .max_connections(10)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect(db_url)
            .await
            .map_err(DatabaseError::PoolCreationError)?;

        Ok(Database { pool })
    }
}

/// JWT configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> DbResult<Self> {
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| DatabaseError::MissingEnvVar("JWT_SECRET"))?;
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN")
            .map_err(|_| DatabaseError::MissingEnvVar("JWT_EXPIRED_IN"))?;
        let jwt_maxage = std::env::var("JWT_MAXAGE")
            .map_err(|_| DatabaseError::MissingEnvVar("JWT_MAXAGE"))?;

        Ok(Config {
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>()
                .map_err(|_| DatabaseError::InvalidEnvVar("JWT_MAXAGE"))?,
        })
    }
}
