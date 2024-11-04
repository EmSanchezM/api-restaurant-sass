use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::value_objects::{surreal_id::SurrealId, address::Address, emergency_contact::EmergencyContact};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
  #[serde(rename = "id")]
  pub surreal_id: SurrealId,
  pub user_id: SurrealId,
  pub first_name: String,
  pub last_name: String,
  pub phone: String,
  pub address: Address,
  pub position: Option<String>,
  pub birth_date: DateTime<Utc>,
  pub avatar: Option<String>,
  pub emergency_contact: Option<EmergencyContact>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}