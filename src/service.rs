use actix_web::{web, Scope};

mod shift_problem;
mod department;
mod spare_part;
mod relations;
mod employee;
mod problem;
mod machine;
mod syncing;
mod shift;
mod note;

pub fn scopes() -> Scope {
  web::scope("/api")
        .service(shift_problem::scope())
        .service(department::scope())
        .service(spare_part::scope())
        .service(relations::scope())
        .service(employee::scope())
        .service(problem::scope())
        .service(machine::scope())
        .service(syncing::scope())
        .service(shift::scope())
        .service(note::scope())
}
