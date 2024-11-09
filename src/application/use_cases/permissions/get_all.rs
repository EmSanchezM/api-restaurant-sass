use crate::domain::repositories::permission_repository::PermissionRepository;
use crate::application::dtos::permissions::permission_response::PermissionResponse;
use crate::domain::error::Error;

pub struct GetAllPermissionsUseCase<T> where T: PermissionRepository {
  permission_repository: T,
}

impl<T> GetAllPermissionsUseCase<T> where T: PermissionRepository {
  pub fn new(permission_repository: T) -> Self {
    Self { permission_repository }
  }

  pub async fn execute(&self) -> Result<Vec<PermissionResponse>, Error> {
    let permissions = self.permission_repository.find_all().await?;

    Ok(permissions.iter().map(|permission| PermissionResponse {
      id: permission.surreal_id.id().to_string(),
      name: permission.name.clone(),
      description: permission.description.clone(),
      resource: permission.resource.to_string(),
      action: permission.action.to_string(),
      is_active: permission.is_active,
      created_at: permission.created_at,
    }).collect())
  }
}
