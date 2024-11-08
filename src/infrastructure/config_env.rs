use crate::domain::error::Error;
use crate::domain::services::token::TokenConfig;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
  pub token_config: TokenConfig,
}

impl Config {
  pub fn from_env() -> Result<Self, Error> {
    Ok(Self {
      token_config: TokenConfig {
        access_token_secret: std::env::var("ACCESS_TOKEN_SECRET")
          .map_err(|_| Error::ConfigError("ACCESS_TOKEN_SECRET not set".to_string()))?,
        refresh_token_secret: std::env::var("REFRESH_TOKEN_SECRET")
          .map_err(|_| Error::ConfigError("REFRESH_TOKEN_SECRET not set".to_string()))?,
        access_token_duration: Duration::from_secs(30*60), // 30 minutos en segundos
        refresh_token_duration: Duration::from_secs(7 *24 * 60 * 60), // 7 días en segundos
      },
    })
  }
}