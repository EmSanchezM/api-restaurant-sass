use actix_web::web;
use crate::presentation::handlers::auth_handler::{register_handler, login_handler, logout_handler};

//TODO: Agregar middleware para validar token solo en ruta para logout
pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api/v1/auth")
      .service(register_handler)
      .service(login_handler)
      .service(logout_handler)
  );
}