use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use uuid::Uuid;

use crate::domain::entities::token::RefreshToken;
use crate::domain::entities::user::User;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::error::Error;

#[derive(Debug)]
pub struct TokenPair {
  pub access_token: String,
  pub refresh_token: RefreshToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
  pub sub: String,         // user id
  pub email: String,
  pub roles: Vec<String>,
  pub permissions: Option<Vec<String>>,
  pub exp: i64,           // expiration timestamp
  pub iat: i64,           // issued at timestamp
}

#[derive(Debug, Clone)]
pub struct TokenConfig {
  pub access_token_secret: String,
  pub refresh_token_secret: String,
  pub access_token_duration: Duration,
  pub refresh_token_duration: Duration,
}

#[derive(Debug, Clone)]
pub struct TokenService {
  config: TokenConfig,
}

impl TokenService {
  pub fn new(config: TokenConfig) -> Self {
    Self { config }
  }

  pub fn generate_token_pair(&self, user: &User) -> Result<TokenPair, Error> {
    let access_token = self.generate_access_token(user)?;
    let refresh_token = self.generate_refresh_token(user)?;
    
    Ok(TokenPair {
      access_token,
      refresh_token,
    })
  }

  pub fn generate_access_token(&self, user: &User) -> Result<String, Error> {
    let expiration = Utc::now() + self.config.access_token_duration;
    
    let claims = TokenClaims {
      sub: user.surreal_id.id().to_string(),
      email: user.email.clone(),
      roles: user.roles.as_ref().map(|r| r.iter().map(|role| role.name.clone()).collect()).unwrap_or_default(),
      permissions: user.permissions.as_ref().map(|p| p.iter().map(|perm| perm.name.clone()).collect()),
      exp: expiration.timestamp(),
      iat: Utc::now().timestamp(),
    };
  
    encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(self.config.access_token_secret.as_bytes()),
    )
    .map_err(|e| Error::TokenGenerationError(e.to_string()))
  }

  pub fn generate_refresh_token(&self, user: &User) -> Result<RefreshToken, Error> {
    let refresh_token = RefreshToken {
      surreal_id: SurrealId::generate("refresh_token"),
      user_id: user.surreal_id.clone(),
      access_token: Uuid::new_v4().to_string(),
      used: false,
      invalidated: false,
      expires_at: Utc::now() + self.config.refresh_token_duration,
      token: Uuid::new_v4().to_string(),
      created_at: Utc::now(),
    };

    Ok(refresh_token)
  }

  pub fn verify_access_token(&self, token: &str) -> Result<TokenClaims, Error> {
    let validation = Validation::default();

    let claims = decode::<TokenClaims>(
      token,
      &DecodingKey::from_secret(self.config.access_token_secret.as_bytes()),
      &validation,
    )
    .map_err(|e| match e.kind() {
      jsonwebtoken::errors::ErrorKind::ExpiredSignature => Error::TokenExpired,
      _ => Error::InvalidToken,
    })?;

    Ok(claims.claims)
  }

  pub fn is_token_expired(&self, claims: &TokenClaims) -> bool {
    claims.exp < Utc::now().timestamp()
  }
}