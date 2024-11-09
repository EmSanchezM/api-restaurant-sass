use actix_web::{post, web, HttpResponse};

use crate::application::dtos::register::register_request::RegisterRequest;
use crate::application::dtos::login::login_request::LoginRequest;

use crate::infrastructure::repositories::{
  surreal_user_repository::SurrealUserRepository,
  surreal_token_repository::SurrealTokenRepository
};

use crate::infrastructure::config_env::Config;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;
use crate::domain::services::token::TokenService;

use crate::application::use_cases::auth::{
  login::LoginUseCase,
  register::RegisterUseCase,
  logout::LogoutUseCase
};

#[post("/register")]
pub async fn register_handler(
  config: web::Data<Config>,
  db_connection: web::Data<DatabaseConnection>,
  request: web::Json<RegisterRequest>
) -> HttpResponse {

  let token_service = TokenService::new(config.token_config.clone());
  let repo = SurrealUserRepository::new(&db_connection);
  let token_repo = SurrealTokenRepository::new(&db_connection);
  
    match RegisterUseCase::new(repo, token_repo, token_service)
      .execute(request.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => {
          //Todo: Log de errores
          //error!("Error en el registro: {:?}", err);
          HttpResponse::InternalServerError().body("Error al registrar usuario")
        }
      }
}

#[post("/login")]
pub async fn login_handler(
  config: web::Data<Config>,
  db_connection: web::Data<DatabaseConnection>,
  request: web::Json<LoginRequest>
) -> HttpResponse {
  let repo = SurrealUserRepository::new(&db_connection);
  let token_repo = SurrealTokenRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());
  
  match LoginUseCase::new(repo, token_repo, token_service)
    .execute(request.into_inner()).await {
      Ok(response) => HttpResponse::Ok().json(response),
      Err(_) => {
        //Todo: Log de errores
        //error!("Error en el login: {:?}", err);
        HttpResponse::Unauthorized().body("Credenciales inválidas")
      }
    }
}

#[post("/logout")]
pub async fn logout_handler(
  config: web::Data<Config>,
  db_connection: web::Data<DatabaseConnection>,
) -> HttpResponse {
  let repo = SurrealTokenRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());

  //Todo: Obtener el token del header
  let token = "token";

  match LogoutUseCase::new(repo, token_service).execute(token).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::Unauthorized().body("Token inválido")
  }
}