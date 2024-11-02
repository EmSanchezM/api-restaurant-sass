use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub struct DatabaseConnection {
  client: Surreal<Ws>,
}

impl DatabaseConnection {
  pub async fn new(url: &str, user: &str, pass: &str) -> Result<Self, Error> {
    let client = Surreal::new::<Ws>(url).await?;
    client.signin(Root {
        username: user,
        password: pass,
    })
    .await?;
    
    Ok(Self { client })
  }

  pub fn get_client(&self) -> &Surreal<Ws> {
    &self.client
  }
}