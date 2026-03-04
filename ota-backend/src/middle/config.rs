use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Environment variable {0} must be set")]
    MissingEnvVar(&'static str),
    #[error("Invalid environment variable {0}")]
    InvalidEnvVar(&'static str),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Result<Config, ConfigError> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL"))?;
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| ConfigError::MissingEnvVar("JWT_SECRET"))?;
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN")
            .map_err(|_| ConfigError::MissingEnvVar("JWT_EXPIRED_IN"))?;
        let jwt_maxage = std::env::var("JWT_MAXAGE")
            .map_err(|_| ConfigError::MissingEnvVar("JWT_MAXAGE"))?;

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>()
                .map_err(|_| ConfigError::InvalidEnvVar("JWT_MAXAGE"))?,
        })
    }
}
