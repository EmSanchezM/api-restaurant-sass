use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::repositories::role_repository::RoleRepository;
use crate::application::dtos::roles::role_response::RoleResponse;
use crate::domain::error::Error;

pub struct GetRoleByIdUseCase<T> where T: RoleRepository {
  role_repository: T,
}

impl<T> GetRoleByIdUseCase<T> where T: RoleRepository {
  pub fn new(role_repository: T) -> Self {
    Self { role_repository }
  }

  pub async fn execute(&self, id: &str) -> Result<RoleResponse, Error> {
    let role_id = SurrealId::new("role", id);
    let role = self.role_repository.find_by_id(&role_id).await?;

    if role.is_none() {
      return Err(Error::RoleNotFound);
    }

    let role = role.unwrap();

    Ok(RoleResponse {
      id: role.surreal_id.id().to_string(),
      name: role.name,
      description: role.description,
      hierarchy_level: role.hierarchy_level,
      is_active: role.is_active,
      created_at: role.created_at,
    })
  }
}