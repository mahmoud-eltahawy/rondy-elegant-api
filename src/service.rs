use actix_web::{web, Scope};

mod task;
mod employee;

pub fn scopes() -> Scope {
  web::scope("/api")
        .service(task::scope())
        .service(employee::scope())
}
