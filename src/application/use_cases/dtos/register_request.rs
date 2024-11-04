pub struct RegisterRequest {
  pub email: String,
  pub password: String,
  pub user_type: UserType,
  pub created_by: Option<String>,
}