use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
  pub password: String,
}
