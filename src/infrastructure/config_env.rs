use crate::domain::error::Error;
use crate::domain::services::token::TokenConfig;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
  pub url: String,
  pub namespace: String,
  pub database: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
  pub host: String,
  pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Config {
  pub token_config: TokenConfig,
  pub database: DatabaseConfig,
  pub server: ServerConfig,
}

impl Config {
  pub fn from_env() -> Result<Self, Error> {
    Ok(Self {
      token_config: TokenConfig {
        access_token_secret: std::env::var("ACCESS_TOKEN_SECRET")
          .map_err(|_| Error::ConfigError("ACCESS_TOKEN_SECRET not set".to_string()))?,
        access_token_duration: Duration::from_secs(30*60), // 30 minutos en segundos
        refresh_token_duration: Duration::from_secs(7 *24 * 60 * 60), // 7 días en segundos
      },
      database: DatabaseConfig {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        namespace: std::env::var("DATABASE_NAMESPACE").expect("SURREAL_DB_NAMESPACE must be set"),
        database: std::env::var("DATABASE_DATABASE").expect("SURREAL_DB_DATABASE must be set"),
      },
      server: ServerConfig {
        host: std::env::var("SERVER_HOST").expect("SERVER_HOST must be set"),
        port: std::env::var("SERVER_PORT").expect("SERVER_PORT must be set").parse().expect("SERVER_PORT must be a valid number"),
      },
    })
  }
}