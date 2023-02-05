use actix_web::{
  Scope,
  web,
  HttpResponse,
  post,get, Responder
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{
  repo::shift_problem::{
    save_shift_problem,
    fetch_department_shift_problems_by_writer_and_shift_id
  },
  AppState
};
use rec::model::shift_problem::MinimamlShiftProblem;


pub fn scope() -> Scope{
  web::scope("/sp")
    .service(save)
    .service(get_current_shift_problems)
}

#[post("/save")]
async fn save(state : web::Data<AppState>, shift_problem : web::Json<MinimamlShiftProblem>) -> impl Responder{
  match save_shift_problem(&state, shift_problem.into_inner()).await {
    Ok(shift_problem_id) => HttpResponse::Ok().json(Some(shift_problem_id)),
    Err(_)             => HttpResponse::NotFound().json(None::<Uuid>)
  }
}

#[derive(Serialize,Deserialize)]
struct WriterAndShiftIds{
  writer_id : Uuid,
  shift_id  : Uuid
}

#[get("/cproblems")]
async fn get_current_shift_problems(state : web::Data<AppState>,
                                    ids : web::Json<WriterAndShiftIds>) -> impl Responder{
  let WriterAndShiftIds{writer_id,shift_id} = ids.into_inner();
  match fetch_department_shift_problems_by_writer_and_shift_id(&state, writer_id, shift_id).await {
    Ok(shift_problems) => HttpResponse::Ok().json(Some(shift_problems)),
    Err(_)             => HttpResponse::NotFound().json(None::<Vec<MinimamlShiftProblem>>)
  }
}
