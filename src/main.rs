mod model;
mod service;
mod repo;
mod config;

use config::{ get_config_postgres_url, get_configs_server, set_debug_configs};
use dotenv::dotenv;

use actix_web::{HttpServer, middleware::Logger, App, web::Data};

use service::task::test;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct AppState{
   pub db : Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  set_debug_configs();

  let pool = PgPoolOptions::new()
      .max_connections(10)
      .connect(&get_config_postgres_url())
      .await
      .expect("failed to connect db");

  HttpServer::new(move || {
      let logger = Logger::default();
      App::new()
          .app_data(Data::new(AppState{db: pool.clone()}))
          .wrap(logger)
          .service(test)
  }).bind(get_configs_server())?
      .run()
      .await?;
  Ok(())
}
