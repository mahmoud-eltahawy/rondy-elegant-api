use actix_web::{web, Scope};

pub mod task;

pub fn scopes() -> Scope {
  web::scope("/api")
        .service(task::scope())
}
