use crate::domain::repositories::role_repository::RoleRepository;
use crate::application::dtos::roles::role_response::RoleResponse;
use crate::domain::error::Error;

pub struct GetAllRolesUseCase<T> where T: RoleRepository {
  role_repository: T,
}

impl<T> GetAllRolesUseCase<T> where T: RoleRepository {
  pub fn new(role_repository: T) -> Self {
    Self { role_repository }
  }

  pub async fn execute(&self) -> Result<Vec<RoleResponse>, Error> {
    let roles = self.role_repository.find_all().await?;

    Ok(roles.iter().map(|role| RoleResponse {
      id: role.surreal_id.id().to_string(),
      name: role.name.clone(),
      description: role.description.clone(),
      hierarchy_level: role.hierarchy_level,
      is_active: role.is_active,
      created_at: role.created_at,
    }).collect())
  }
}
