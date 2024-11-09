use std::str::FromStr;

use crate::domain::entities::permission::Permission;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::repositories::permission_repository::PermissionRepository;
use crate::domain::error::Error;
use crate::domain::entities::permission::{Resource, Action};
use crate::application::dtos::permissions::update_permission_request::UpdatePermissionRequest;
use crate::application::dtos::permissions::permission_response::PermissionResponse;

pub struct UpdatePermissionUseCase<T> where T: PermissionRepository {
  permission_repository: T,
}

impl<T> UpdatePermissionUseCase<T> where T: PermissionRepository {
  pub fn new(permission_repository: T) -> Self {
    Self { permission_repository }
  }

  pub async fn execute(&self, id: &str, request: &UpdatePermissionRequest) -> Result<PermissionResponse, Error> {
    let permission_id = SurrealId::new("permission", id);
    let permission = self.permission_repository.find_by_id(&permission_id).await?;

    match permission {
      None => return Err(Error::PermissionNotFound),
      Some(permission) => {
        let resource = Resource::from_str(&request.resource.clone().unwrap()).map_err(|_| Error::InvalidResource)?;
        let action = Action::from_str(&request.action.clone().unwrap()).map_err(|_| Error::InvalidAction)?;

        let payload = Permission::new(
          request.name.clone().unwrap(),
          request.description.clone().unwrap(),
          resource,
          action
        );
        let updated_permission = self.permission_repository.update(&permission.surreal_id, &payload).await?;
        
        Ok(PermissionResponse {
          id: updated_permission.surreal_id.id().to_string(),
          name: updated_permission.name,
          description: updated_permission.description,
          resource: updated_permission.resource.to_string(),
          action: updated_permission.action.to_string(),
          is_active: updated_permission.is_active,
          created_at: updated_permission.created_at,
        })
      }
    }
  }
}
