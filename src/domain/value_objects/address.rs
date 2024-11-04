use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
  pub street: String,
  pub city: String,
  pub state: String,
  pub country: String,
  pub postal_code: String,
}