use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::surreal_id::SurrealId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "id")]
  pub surreal_id: SurrealId,
  pub email: String,
  pub password: String,
  pub status: UserStatus,
  pub user_type: UserType,
  pub is_verified: bool,
  pub failed_login_attempts: i32,
  pub last_login: Option<DateTime<Utc>>,
  pub locked_until: Option<DateTime<Utc>>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
  Active,
  Inactive,
  Suspended,
  PendingVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserType {
  Customer,
  Employee,
  Admin,
}