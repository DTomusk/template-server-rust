use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_minutes: i64,
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidPort(std::num::ParseIntError),
    MissingDatabaseUrl,
    MissingJwtSecret,
    MissingJwtExpirationMinutes,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPort(_) => write!(f, "PORT must be a valid u16"),
            Self::MissingDatabaseUrl => write!(f, "DATABASE_URL must be set"),
            Self::MissingJwtSecret => write!(f, "JWT_SECRET must be set"),
            Self::MissingJwtExpirationMinutes => write!(f, "JWT_EXPIRATION_MINUTES must be set"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidPort(err) => Some(err),
            Self::MissingDatabaseUrl => None,
            Self::MissingJwtSecret => None,
            Self::MissingJwtExpirationMinutes => None,
        }
    }
}

// impl is used to define methods on a type
impl Config {
    // doesn't take self as an argument, so it's an associated function (like a static method)
    pub fn from_env() -> Result<Self, ConfigError> {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(ConfigError::InvalidPort)?;
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingDatabaseUrl)?;
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| ConfigError::MissingJwtSecret)?;
        let jwt_expiration_minutes = env::var("JWT_EXPIRATION_MINUTES")
            .map_err(|_| ConfigError::MissingJwtExpirationMinutes)?
            .parse()
            .map_err(|e| ConfigError::InvalidPort(e))?; // reuse InvalidPort error for simplicity
        Ok(Self { port, database_url, jwt_secret, jwt_expiration_minutes })
    }
}