use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: DbPool,
    pub env: Config,
}

impl Database {
    pub fn new(db_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool: DbPool = r2d2::Pool::builder()
            .min_idle(Some(5)) // 最小空闲连接数
            .max_size(10) // 最大连接数
            .connection_timeout(std::time::Duration::from_secs(30))
            .build(manager)
            .expect("Failed to create pool.");

        Database {
            pool,
            env: Config::init(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        Config {
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}
