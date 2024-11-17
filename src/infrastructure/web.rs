use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use crate::infrastructure::config_env::Config;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;
use crate::presentation::routes;

pub async fn run(config: Config) -> std::io::Result<()> {

  let config_arc = Arc::new(config);
  let config_arc_db = config_arc.clone();
  let config = config_arc.clone();

  let address = (config_arc.server.host.as_str(), config_arc.server.port);

  let database_connection = Arc::new(DatabaseConnection::new(&config_arc_db.database.url, &config_arc_db.database.namespace, &config_arc_db.database.database)
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?);
  
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(database_connection.clone()))
      .app_data(web::Data::new(config.clone()))
      .wrap(Logger::default())
      .configure(routes::auth_routes::routes)
      .configure(routes::role_routes::routes)
      .configure(routes::permission_routes::routes)
      .configure(routes::profile_routes::routes)
      .configure(routes::health_check_routes::routes)
  })
  .bind(address)
  .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
  .run()
  .await
}