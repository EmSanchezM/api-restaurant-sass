use crate::domain::entities::role::Role;
use crate::domain::repositories::role_repository::RoleRepository;
use crate::application::dtos::roles::create_role_request::CreateRoleRequest;
use crate::application::dtos::roles::role_response::RoleResponse;
use crate::domain::error::Error;

pub struct CreateRoleUseCase<T> where T: RoleRepository {
  role_repository: T,
}

impl<T> CreateRoleUseCase<T> where T: RoleRepository {
  pub fn new(role_repository: T) -> Self {
    Self { role_repository }
  }

  pub async fn execute(&self, request: &CreateRoleRequest) -> Result<RoleResponse, Error> {
    let new_role = Role::new(request.name.clone(), request.description.clone(), request.hierarchy_level.clone());

    let role = self.role_repository.create(&new_role).await?;

    Ok(RoleResponse { 
      id: role.surreal_id.to_string(),
      name: role.name,
      description: role.description,
      hierarchy_level: role.hierarchy_level,
      is_active: role.is_active,
      created_at: role.created_at
    })
  }
}