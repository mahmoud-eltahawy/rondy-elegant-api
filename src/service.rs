use actix_web::{web, Scope};

mod shift_problem;
mod spare_part;
mod relations;
mod employee;
mod problem;
mod machine;
mod syncing;
mod shift;

pub fn scopes() -> Scope {
  web::scope("/api")
        .service(shift_problem::scope())
        .service(spare_part::scope())
        .service(employee::scope())
        .service(problem::scope())
        .service(machine::scope())
        .service(syncing::scope())
        .service(relations::scope())
        .service(shift::scope())
}
