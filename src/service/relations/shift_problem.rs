use actix_web::{Scope, web::{self, Data}, post, Responder, HttpResponse};
use rec::{
  model::relations::{
      ShiftProblemProblem,
      ShiftProblemSparePart
    },
  crud_sync::{
    CudVersion,
    Cud,Table
  }
};

use crate::{
  AppState,
  repo::{relations::shift_problem::{
    save_problem_to_shift_problem,
    remove_problem_from_shift_problem,
    save_spare_part_to_shift_problem,
    remove_spare_part_from_shift_problem
  }, syncing::record_version}
};


pub fn scope() -> Scope {
  web::scope("/sp")
    .service(save_problem)
    .service(delete_problem)
    .service(save_spare_part)
    .service(delete_spare_part)
}

#[post("/p-save")]
async fn save_problem(state : Data<AppState>,sp : web::Json<ShiftProblemProblem>) -> impl Responder{
  let sp = sp.into_inner();
  match save_problem_to_shift_problem(&state,&sp).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblemProblem,
        version_number : 0,
        target_id : sp.problem_id,
        other_target_id: Some(sp.shift_problem_id)
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/p-delete")]
async fn delete_problem(state : Data<AppState>,sp : web::Json<ShiftProblemProblem>) -> impl Responder{
  let sp = sp.into_inner();
  match remove_problem_from_shift_problem(&state,&sp).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblemProblem,
        version_number : 0,
        target_id : sp.problem_id,
        other_target_id: Some(sp.shift_problem_id)
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/s-save")]
async fn save_spare_part(state : Data<AppState>,ss : web::Json<ShiftProblemSparePart>) -> impl Responder{
  let ss = ss.into_inner();
  match save_spare_part_to_shift_problem(&state,&ss).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblemSparePart,
        version_number : 0,
        target_id : ss.spare_part_id,
        other_target_id: Some(ss.shift_problem_id)
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/s-delete")]
async fn delete_spare_part(state : Data<AppState>,ss : web::Json<ShiftProblemSparePart>) -> impl Responder{
  let ss = ss.into_inner();
  match remove_spare_part_from_shift_problem(&state,&ss).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblemSparePart,
        version_number : 0,
        target_id : ss.spare_part_id,
        other_target_id: Some(ss.shift_problem_id)
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}
