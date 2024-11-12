use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::emergency_contact::EmergencyContact;
use crate::domain::value_objects::address::Address;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub phone: Option<String>,
  pub address: Option<Address>,
  pub position: Option<String>,
  pub birth_date: Option<DateTime<Utc>>,
  pub avatar: Option<String>,
  pub emergency_contact: Option<EmergencyContact>,
}
