use std::str::FromStr;

use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::PermissionRepository;
use crate::application::dtos::permissions::create_permission_request::CreatePermissionRequest;
use crate::application::dtos::permissions::permission_response::PermissionResponse;
use crate::domain::error::Error;
use crate::domain::entities::permission::{Resource, Action};

pub struct CreatePermissionUseCase<T> where T: PermissionRepository {
  permission_repository: T,
}

impl<T> CreatePermissionUseCase<T> where T: PermissionRepository {
  pub fn new(permission_repository: T) -> Self {
    Self { permission_repository }
  }

  pub async fn execute(&self, request: &CreatePermissionRequest) -> Result<PermissionResponse, Error> {
    let resource = Resource::from_str(&request.resource).map_err(|_| Error::InvalidResource)?;
    let action = Action::from_str(&request.action).map_err(|_| Error::InvalidAction)?;

    let new_permission = Permission::new(
      request.name.clone(),
      request.description.clone(),
      resource,
      action
    );

    let permission = self.permission_repository.create(&new_permission).await?;

    Ok(PermissionResponse { 
      id: permission.surreal_id.to_string(),
      name: permission.name,
      description: permission.description,
      resource: permission.resource.to_string(),
      action: permission.action.to_string(),
      is_active: permission.is_active,
      created_at: permission.created_at
    })
  }
}