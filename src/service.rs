use actix_web::{web, Scope};

mod problem;
mod employee;
mod spare_part;
mod problem_detail;
mod machine;

pub fn scopes() -> Scope {
  web::scope("/api")
        .service(problem_detail::scope())
        .service(problem::scope())
        .service(employee::scope())
        .service(spare_part::scope())
        .service(machine::scope())
}
