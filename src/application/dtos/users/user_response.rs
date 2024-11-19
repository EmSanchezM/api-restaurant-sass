use serde::{Deserialize, Serialize};

use crate::{application::dtos::profile::profile_response::ProfileResponse, domain::value_objects::{user_status::UserStatus, user_types::UserType}};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
  pub id: String,
  pub user_type: UserType,
  pub status: UserStatus,
  pub failed_login_attempts: i32,
  pub last_login: Option<String>,
  pub locked_until: Option<String>,
  pub profile: Option<ProfileResponse>,
  pub email: String,
  pub is_active: bool,
  pub created_at: String,
}
