use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
  pub user_id: String,
  pub email: String,
  pub access_token: String,
  pub refresh_token: String,
  pub access_token_expires_at: DateTime<Utc>,
}