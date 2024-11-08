mod application;
mod domain;
mod infrastructure;

use infrastructure::config_env::Config;
use domain::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Error> {
  let config = Config::from_env()?;
  println!("{:?}", config);

  Ok(())
}
