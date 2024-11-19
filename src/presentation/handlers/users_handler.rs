use actix_web::{post, get, web, HttpResponse};

use crate::application::dtos::users::{
  change_password_request::ChangePasswordRequest,
  verification_status_user_request::VerificationStatusUserRequest
};

use crate::application::use_cases::users::{
  change_password::ChangePasswordUseCase,
  verification_user::VerificationUserUseCase,
  disable_user::DisableUserUseCase,
  get_all::GetAllUsersUseCase,
};

use crate::infrastructure::repositories::surreal_user_repository::SurrealUserRepository;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;
use crate::domain::services::token::TokenService;
use crate::infrastructure::config_env::Config;

#[get("/")]
pub async fn get_all_users_handler(
  db_connection: web::Data<DatabaseConnection>,
  config: web::Data<Config>,
) -> HttpResponse {
  let repo = SurrealUserRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());

  let token = "token";
  
  match GetAllUsersUseCase::new(repo, token_service).execute(token).await {
    Ok(users) => HttpResponse::Ok().json(users),
    Err(_) => HttpResponse::InternalServerError().body("Error al obtener usuarios")
  }
}

#[post("/verification")]
pub async fn verification_user_handler(
  db_connection: web::Data<DatabaseConnection>,
  config: web::Data<Config>,
  request: web::Json<VerificationStatusUserRequest>
) -> HttpResponse {
  let repo = SurrealUserRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());

  let token = "token";
  
  match VerificationUserUseCase::new(repo, token_service).execute(token, &request.into_inner()).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().body("Error al verificar usuario")
  }
}

#[post("/change-password")]
pub async fn change_password_handler(
  db_connection: web::Data<DatabaseConnection>,
  config: web::Data<Config>,
  request: web::Json<ChangePasswordRequest>
) -> HttpResponse {
  let repo = SurrealUserRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());

  let token = "token";

  match ChangePasswordUseCase::new(repo, token_service).execute(token, &request.into_inner()).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().body("Error al cambiar la contraseña")
  }
}

#[post("/disable")]
pub async fn disable_user_handler(
  db_connection: web::Data<DatabaseConnection>,
  config: web::Data<Config>,
) -> HttpResponse {
  let repo = SurrealUserRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());

  let token = "token";
  
  match DisableUserUseCase::new(repo, token_service).execute(token).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().body("Error al deshabilitar usuario")
  }
}