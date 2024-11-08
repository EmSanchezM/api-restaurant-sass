use crate::domain::entities::user::User;
use crate::domain::entities::token::RefreshToken;

pub enum ProfileStatus {
  PendingCompletion,
  PendingApproval,
}

pub struct RegisterResponse {
  pub user: User,
  pub refresh_token: RefreshToken,
  pub profile_status: ProfileStatus,
}
