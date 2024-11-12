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
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Profile {
  pub fn new(user_id: SurrealId, first_name: String, last_name: String, phone: String, address: Address, position: Option<String>, avatar: Option<String>, emergency_contact: Option<EmergencyContact>, birth_date: DateTime<Utc>) -> Self {
    Self { 
      surreal_id: SurrealId::generate("profile"),
      user_id,
      first_name,
      last_name,
      phone,
      address,
      position,
      birth_date,
      avatar,
      emergency_contact,
      is_active: true,
      created_at: Utc::now(),
      updated_at: Utc::now()
    }
  }
}