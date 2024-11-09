use serde::{Deserialize, Serialize};

use crate::domain::entities::user::User;
use crate::domain::entities::token::RefreshToken;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProfileStatus {
  PendingCompletion,
  PendingApproval,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
  pub user: User,
  pub refresh_token: RefreshToken,
  pub profile_status: ProfileStatus,
}
