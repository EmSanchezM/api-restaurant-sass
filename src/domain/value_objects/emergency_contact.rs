use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyContact {
  pub name: String,
  pub phone: String,
  pub relationship: String,
}

impl EmergencyContact {
  pub fn new(name: String, phone: String, relationship: String) -> Self {
    Self { name, phone, relationship }
  }
}