use actix_web::{
  Scope,
  web,
  HttpResponse,
  post,get, Responder
};
use uuid::Uuid;

use crate::{
  repo::shift_problem::{
    save_shift_problem,
    fetch_department_shift_problems_by_writer_and_shift_id, delete_db_shift_problem, update_db_shift_problem, fetch_shift_problem_by_id
  },
  AppState
};
use rec::model::shift_problem::{MinimamlShiftProblem,WriterAndShiftIds, DbShiftProblem};


pub fn scope() -> Scope{
  web::scope("/sp")
    .service(save)
    .service(get_current_shift_problems)
    .service(update_db_problem)
    .service(delete)
    .service(get_problem_by_id)
}

#[post("/save")]
async fn save(state : web::Data<AppState>, shift_problem : web::Json<MinimamlShiftProblem>) -> impl Responder{
  match save_shift_problem(&state, shift_problem.into_inner()).await {
    Ok(shift_problem_id) => HttpResponse::Ok().json(shift_problem_id),
    Err(_)             => HttpResponse::InternalServerError().into()
  }
}

#[post("/update")]
async fn update_db_problem(state : web::Data<AppState>, shift_problem : web::Json<DbShiftProblem>) -> impl Responder{
  match update_db_shift_problem(&state, &shift_problem.into_inner()).await {
    Ok(_)   => HttpResponse::Ok(),
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/delete")]
async fn delete(state : web::Data<AppState>, shift_problem_id : web::Json<Uuid>) -> impl Responder{
  match delete_db_shift_problem(&state, &shift_problem_id.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_)             => HttpResponse::InternalServerError()
  }
}

#[get("/cproblems")]
async fn get_current_shift_problems(state : web::Data<AppState>,
                                    ids : web::Json<WriterAndShiftIds>) -> impl Responder{
  let WriterAndShiftIds{writer_id,shift_id} = ids.into_inner();
  match fetch_department_shift_problems_by_writer_and_shift_id(&state, writer_id, shift_id).await {
    Ok(shift_problems) => HttpResponse::Ok().json(shift_problems),
    Err(_)             => HttpResponse::InternalServerError().into()
  }
}

#[post("/sp")]
async fn get_problem_by_id(state : web::Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  match fetch_shift_problem_by_id(&state, id.into_inner()).await {
    Ok(problem) => HttpResponse::Ok().json(problem),
    Err(_)      => HttpResponse::InternalServerError().into()
  }
}
