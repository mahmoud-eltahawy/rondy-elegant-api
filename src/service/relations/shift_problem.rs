use actix_web::{Scope, web::{self, Data}, post, Responder, HttpResponse};
use rec::model::{note::DbNote, relations::{ShiftProblemProblem, ShiftProblemSparePart}};

use crate::{AppState, repo::relations::shift_problem::{save_note_to_shift_problem, remove_note_to_shift_problem, save_problem_to_shift_problem, remove_problem_from_shift_problem, save_spare_part_to_shift_problem, remove_spare_part_from_shift_problem}};


pub fn scope() -> Scope{
  web::scope("/sp")
    .service(save_note)
    .service(save_problem)
    .service(save_spare_part)
    .service(delete_note)
    .service(delete_problem)
    .service(delete_spare_part)
}

#[post("/note-save")]
async fn save_note(state : Data<AppState>,note : web::Json<DbNote>) -> impl Responder{
  match save_note_to_shift_problem(&state,note.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/note-delete")]
async fn delete_note(state : Data<AppState>,note : web::Json<DbNote>) -> impl Responder{
  match remove_note_to_shift_problem(&state,note.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/problem-save")]
async fn save_problem(state : Data<AppState>,sp : web::Json<ShiftProblemProblem>) -> impl Responder{
  match save_problem_to_shift_problem(&state,sp.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/problem-delete")]
async fn delete_problem(state : Data<AppState>,sp : web::Json<ShiftProblemProblem>) -> impl Responder{
  match remove_problem_from_shift_problem(&state,sp.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/spare-part-save")]
async fn save_spare_part(state : Data<AppState>,ss : web::Json<ShiftProblemSparePart>) -> impl Responder{
  match save_spare_part_to_shift_problem(&state,ss.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/spare-part-delete")]
async fn delete_spare_part(state : Data<AppState>,ss : web::Json<ShiftProblemSparePart>) -> impl Responder{
  match remove_spare_part_from_shift_problem(&state,ss.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}
