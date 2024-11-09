use actix_web::{post, get, put, delete, web, HttpResponse};

use crate::application::dtos::permissions::{
  create_permission_request::CreatePermissionRequest,
  update_permission_request::UpdatePermissionRequest,
}; 

use crate::application::use_cases::permissions::{
  create::CreatePermissionUseCase,
  update::UpdatePermissionUseCase,
  get_all::GetAllPermissionsUseCase,
  get_by_id::GetPermissionByIdUseCase,
  remove::RemovePermissionUseCase
};

use crate::infrastructure::repositories::surreal_permission_repository::SurrealPermissionRepository;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;

#[post("/permissions")]
pub async fn create_permission_handler(
  db_connection: web::Data<DatabaseConnection>,
  request: web::Json<CreatePermissionRequest>
) -> HttpResponse {
  
  let repo = SurrealPermissionRepository::new(&db_connection);
  
  match CreatePermissionUseCase::new(repo)
    .execute(&request.into_inner()).await {
      Ok(response) => HttpResponse::Ok().json(response),
      Err(_) => {
        //Todo: Log de errores
        //error!("Error en el registro: {:?}", err);
        HttpResponse::InternalServerError().body("Error al crear permiso")
      }
    }
}

#[put("/permissions/{id}")]
pub async fn update_permission_handler(
  db_connection: web::Data<DatabaseConnection>,
  request: web::Json<UpdatePermissionRequest>,
  id: web::Path<String>
) -> HttpResponse {

  let repo = SurrealPermissionRepository::new(&db_connection);

  match UpdatePermissionUseCase::new(repo)
    .execute(&id.into_inner(),&request.into_inner()).await {
      Ok(response) => HttpResponse::Ok().json(response),
      Err(_) => HttpResponse::InternalServerError().body("Error al actualizar permiso")
    }
}

#[delete("/permissions/{id}")]
pub async fn delete_permission_handler(
  db_connection: web::Data<DatabaseConnection>,
  id: web::Path<String>
) -> HttpResponse {

  let repo = SurrealPermissionRepository::new(&db_connection);

  match RemovePermissionUseCase::new(repo).execute(&id.into_inner()).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().body("Error al eliminar permiso")
  }
}

#[get("/permissions")]
pub async fn get_all_permissions_handler(
  db_connection: web::Data<DatabaseConnection>
) -> HttpResponse {
  let repo = SurrealPermissionRepository::new(&db_connection);

  match GetAllPermissionsUseCase::new(repo).execute().await {
    Ok(permissions) => HttpResponse::Ok().json(permissions),
    Err(_) => HttpResponse::InternalServerError().body("Error al obtener permisos")
  }
}

#[get("/permissions/{id}")]
pub async fn get_permission_by_id_handler(
  db_connection: web::Data<DatabaseConnection>,
  id: web::Path<String>
) -> HttpResponse {
  let repo = SurrealPermissionRepository::new(&db_connection);

  match GetPermissionByIdUseCase::new(repo).execute(&id.into_inner()).await {
    Ok(permission) => HttpResponse::Ok().json(permission),
    Err(_) => HttpResponse::InternalServerError().body("Error al obtener permiso")
  }
}