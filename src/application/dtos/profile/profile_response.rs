use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{address::Address, emergency_contact::EmergencyContact};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
  pub id: String,
  pub user_id: String,
  pub first_name: String,
  pub last_name: String,
  pub phone: String,
  pub address: Option<Address>,
  pub position: Option<String>,
  pub birth_date: DateTime<Utc>,
  pub avatar: Option<String>,
  pub emergency_contact: Option<EmergencyContact>,
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
}
