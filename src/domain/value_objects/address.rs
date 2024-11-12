use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
  pub street: String,
  pub city: String,
  pub state: String,
  pub country: String,
  pub postal_code: String,
}

impl Address {
  pub fn new(street: String, city: String, state: String, country: String, postal_code: String) -> Self {
    Self { street, city, state, country, postal_code }
  }
}