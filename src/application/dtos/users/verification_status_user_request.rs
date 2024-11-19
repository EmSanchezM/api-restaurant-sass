use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationStatusUserRequest {
  pub is_verified: bool,
}