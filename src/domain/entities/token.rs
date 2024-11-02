use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::value_objects::surreal_id::SurrealId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
  pub token: String,
  pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
  #[serde(rename = "id")]
  pub surreal_id: SurrealId,
  pub user_id: SurrealId,
  pub token: String,
  pub access_token: String,
  pub expires_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub used: bool,
  pub invalidated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
  pub access_token: AccessToken,
  pub refresh_token: RefreshToken,
}