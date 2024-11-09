use actix_web::web;
use crate::presentation::handlers::permissions_handler::{
  create_permission_handler, get_all_permissions_handler, get_permission_by_id_handler, update_permission_handler, delete_permission_handler
};

//TODO: Agregar middleware para validar token estas rutas seran disponibles solo para usuarios con el rol de super admin
pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api/v1/permissions")
      .service(create_permission_handler)
      .service(get_all_permissions_handler)
      .service(get_permission_by_id_handler)
      .service(update_permission_handler)
      .service(delete_permission_handler)
  );
}