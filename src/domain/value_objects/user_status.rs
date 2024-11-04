use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
  Active,
  Inactive,
  Suspended,
  PendingVerification,
}