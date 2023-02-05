use actix_web::{web, Scope};

mod shift_problem;
mod spare_part;
mod employee;
mod problem;
mod machine;

pub fn scopes() -> Scope {
  web::scope("/api")
        .service(shift_problem::scope())
        .service(spare_part::scope())
        .service(employee::scope())
        .service(problem::scope())
        .service(machine::scope())
}
