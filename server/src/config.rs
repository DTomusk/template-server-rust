use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidPort(std::num::ParseIntError),
    MissingDatabaseUrl,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPort(_) => write!(f, "PORT must be a valid u16"),
            Self::MissingDatabaseUrl => write!(f, "DATABASE_URL must be set"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidPort(err) => Some(err),
            Self::MissingDatabaseUrl => None,
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
        Ok(Self { port, database_url })
    }
}