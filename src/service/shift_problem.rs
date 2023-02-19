use actix_web::{
  web::{Data, self},
  Responder,
  HttpResponse,
  Scope,
  post
};
use rec::{
  model::shift_problem::ShiftProblem,
  crud_sync::{
    CudVersion,Cud,Table
  }
};
use uuid::Uuid;

use crate::{
  AppState,
  repo::{
    syncing::record_version,
    shift_problem::{
      find_shift_problem_by_id,
      save_shift_problem,
      update_shift_problem,
      delete_shift_problem
    }
  }
};

pub fn scope() -> Scope{
  web::scope("/shift-problem")
    .service(get_by_id)
    .service(save)
    .service(update)
    .service(delete)
}
#[post("/shift-problem")]
async fn get_by_id(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  match find_shift_problem_by_id(&state,id.into_inner()).await{
    Ok(problem) => HttpResponse::Ok().json(problem),
    Err(_)      => HttpResponse::InternalServerError().into()
  }
}

#[post("/save")]
async fn save(state : Data<AppState>,problem : web::Json<ShiftProblem>) -> impl Responder{
  let problem = problem.into_inner();
  match save_shift_problem(&state,&problem).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblem,
        version_number : 0,
        target_id : problem.id,
        other_target_id: None
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/update")]
async fn update(state : Data<AppState>,problem : web::Json<ShiftProblem>) -> impl Responder{
  let problem = problem.into_inner();
  match update_shift_problem(&state,&problem).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud             : Cud::Update,
        target_table    : Table::ShiftProblem,
        version_number  : 0,
        target_id       : problem.id,
        other_target_id : None
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/delete")]
async fn delete(state : Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  let id = id.into_inner();
  match delete_shift_problem(&state,&id).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblem,
        version_number : 0,
        target_id : id,
        other_target_id: None
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}
