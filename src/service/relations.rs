mod shift_problem;

use actix_web::{Scope, web};

pub fn scope() -> Scope{
  web::scope("/relation")
        .service(shift_problem::scope())
}
