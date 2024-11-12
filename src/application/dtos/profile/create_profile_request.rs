use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::emergency_contact::EmergencyContact;
use crate::domain::value_objects::address::Address;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProfileRequest {
  pub first_name: String,
  pub last_name: String,
  pub phone: String,
  pub address: Address,
  pub position: Option<String>,
  pub birth_date: DateTime<Utc>,
  pub avatar: Option<String>,
  pub emergency_contact: Option<EmergencyContact>
}
