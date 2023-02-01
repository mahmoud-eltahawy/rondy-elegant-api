use actix_web::{
  Scope,
  web,
  HttpResponse,
  post, Responder
};
use uuid::Uuid;

use crate::{model::problem_detail::MinimamlShiftProblem, repo::problem_detail::save_shift_problem, AppState};


pub fn scope() -> Scope{
  web::scope("/sp")
    .service(save)
}

#[post("/save")]
async fn save(state : web::Data<AppState>, shift_problem : web::Json<MinimamlShiftProblem>) -> impl Responder{
  match save_shift_problem(state, shift_problem.into_inner()).await {
    Ok(shift_problem_id) => HttpResponse::Ok().json(Some(shift_problem_id)),
    Err(_)             => HttpResponse::NotFound().json(None::<Uuid>)
  }
}
