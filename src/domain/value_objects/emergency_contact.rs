use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyContact {
  pub name: String,
  pub phone: String,
  pub relationship: String,
}