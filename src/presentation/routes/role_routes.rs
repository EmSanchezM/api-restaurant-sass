use actix_web::web;
use crate::presentation::handlers::roles_handler::{
  create_role_handler, get_all_roles_handler, get_role_by_id_handler, update_role_handler, delete_role_handler
};

//TODO: Agregar middleware para validar token estas rutas seran disponibles solo para usuarios con el rol de super admin
pub fn routes(config: &mut web::ServiceConfig) {
  print!("Llega a roles_routes");
  
  config.service(
    web::scope("/api/v1/roles")
      .service(create_role_handler)
      .service(get_all_roles_handler)
      .service(get_role_by_id_handler)
      .service(update_role_handler)
      .service(delete_role_handler)
  );
}