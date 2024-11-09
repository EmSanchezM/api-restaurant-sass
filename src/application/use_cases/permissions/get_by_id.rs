use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::repositories::permission_repository::PermissionRepository;
use crate::application::dtos::permissions::permission_response::PermissionResponse;
use crate::domain::error::Error;

pub struct GetPermissionByIdUseCase<T> where T: PermissionRepository {
  permission_repository: T,
}

impl<T> GetPermissionByIdUseCase<T> where T: PermissionRepository {
  pub fn new(permission_repository: T) -> Self {
    Self { permission_repository }
  }

  pub async fn execute(&self, id: &str) -> Result<PermissionResponse, Error> {
    let permission_id = SurrealId::new("permission", id);
    let permission = self.permission_repository.find_by_id(&permission_id).await?;
    
    match permission {
      None => Err(Error::PermissionNotFound),
      Some(permission) => Ok(PermissionResponse {
        id: permission.surreal_id.id().to_string(),
        name: permission.name,
        description: permission.description,
        resource: permission.resource.to_string(),
        action: permission.action.to_string(),
        is_active: permission.is_active,
        created_at: permission.created_at,
      })
    }
  }
}