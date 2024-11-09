use crate::domain::entities::role::Role;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::repositories::role_repository::RoleRepository;
use crate::domain::error::Error;

use crate::application::dtos::roles::update_role_request::UpdateRoleRequest;
use crate::application::dtos::roles::role_response::RoleResponse;

pub struct UpdateRoleUseCase<T> where T: RoleRepository {
  role_repository: T,
}

impl<T> UpdateRoleUseCase<T> where T: RoleRepository {
  pub fn new(role_repository: T) -> Self {
    Self { role_repository }
  }

  pub async fn execute(&self, id: &str, request: &UpdateRoleRequest) -> Result<RoleResponse, Error> {
    let role_id = SurrealId::new("role", id);
    let role = self.role_repository.find_by_id(&role_id).await?;

    if role.is_none() {
      return Err(Error::RoleNotFound);
    }

    let payload = Role::new(request.name.clone().unwrap(), request.description.clone().unwrap(), request.hierarchy_level.unwrap());
    let updated_role = self.role_repository.update(&role.unwrap().surreal_id, &payload).await?;

    Ok(RoleResponse {
      id: updated_role.surreal_id.id().to_string(),
      name: updated_role.name,
      description: updated_role.description,
      hierarchy_level: updated_role.hierarchy_level,
      is_active: updated_role.is_active,
      created_at: updated_role.created_at,
    })
  }
}
