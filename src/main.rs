use dotenv::dotenv;
use env_logger::Env;
use infrastructure::web::run;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use infrastructure::config_env::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  env_logger::Builder
    ::from_env(Env::default().default_filter_or("info"))
    .init();

  let config = Config::from_env().unwrap();

  run(config).await
}