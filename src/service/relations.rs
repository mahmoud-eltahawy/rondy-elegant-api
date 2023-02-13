mod shift_problem;
mod shift;

use actix_web::{Scope, web};

pub fn scope() -> Scope{
  web::scope("/relation")
        .service(shift_problem::scope())
        .service(shift::scope())
}
