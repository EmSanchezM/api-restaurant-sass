use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
  Active,
  Inactive,
  Suspended,
  PendingVerification,
}

impl std::str::FromStr for UserStatus {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "active" => UserStatus::Active,
      "inactive" => UserStatus::Inactive,
      "suspended" => UserStatus::Suspended,
      "pending_verification" => UserStatus::PendingVerification,
      _ => return Err(format!("Invalid user status: {}", s)),
    })
  }
}