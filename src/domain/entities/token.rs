use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
  pub token: String,
  pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
  pub id: Option<Thing>,
  pub user_id: Thing,
  pub token: String,
  pub access_token: String,
  pub expires_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub used: bool,
  pub invalidated: bool,
}

impl RefreshToken {
  pub fn new(user_id: Thing, token: String, access_token: String, expires_at: DateTime<Utc>) -> Self {
    Self { 
      id: None,
      user_id, 
      token, 
      access_token, 
      expires_at, 
      created_at: Utc::now(), 
      used: false, 
      invalidated: false 
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
  pub access_token: AccessToken,
  pub refresh_token: RefreshToken,
}

impl TokenPair {
  pub fn new(access_token: AccessToken, refresh_token: RefreshToken) -> Self {
    Self { access_token, refresh_token }
  }
}