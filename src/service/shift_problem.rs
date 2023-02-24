use actix_web::{
  web::{Data, self},
  Responder,
  HttpResponse,
  Scope,
  post,delete,put,get
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
    },
    relations::shift_problem::{
      save_problem_to_shift_problem,
      save_spare_part_to_shift_problem,
      remove_problem_from_shift_problem,
      remove_spare_part_from_shift_problem
    }
  }
};

pub fn scope() -> Scope{
  web::scope("/sp")
    .service(get_by_id)
    .service(save)
    .service(update)
    .service(delete)
    .service(save_problem)
    .service(delete_problem)
    .service(save_spare_part)
    .service(delete_spare_part)
}

#[get("/{id}")]
async fn get_by_id(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  match find_shift_problem_by_id(&state,id.into_inner()).await{
    Ok(problem) => HttpResponse::Ok().json(problem),
    Err(_)      => HttpResponse::InternalServerError().into()
  }
}

#[delete("/{id}")]
async fn delete(state : Data<AppState>,id : web::Path<Uuid>) -> impl Responder{
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

#[post("/")]
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

#[put("/")]
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

#[get("/problem/{pid}/{spid}")]
async fn save_problem(state : Data<AppState>,path : web::Path<(Uuid,Uuid)>) -> impl Responder{
  let (pid,spid) = path.into_inner();
  match save_problem_to_shift_problem(&state,&pid,&spid).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblemProblem,
        version_number : 0,
        target_id : pid,
        other_target_id: Some(spid)
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[get("/part/{pid}/{spid}")]
async fn save_spare_part(state : Data<AppState>,path : web::Path<(Uuid,Uuid)>) -> impl Responder{
  let (pid,spid) = path.into_inner();
  match save_spare_part_to_shift_problem(&state,&pid,&spid).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblemSparePart,
        version_number : 0,
        target_id : pid,
        other_target_id: Some(spid)
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[delete("/problem/{pid}/{spid}")]
async fn delete_problem(state : Data<AppState>,path : web::Path<(Uuid,Uuid)>) -> impl Responder{
  let (pid,spid) = path.into_inner();
  match remove_problem_from_shift_problem(&state,&pid,&spid).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblemProblem,
        version_number : 0,
        target_id : pid,
        other_target_id: Some(spid)
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[delete("/part/{pid}/{spid}")]
async fn delete_spare_part(state : Data<AppState>,path : web::Path<(Uuid,Uuid)>) -> impl Responder{
  let (pid,spid) = path.into_inner();
  match remove_spare_part_from_shift_problem(&state,&pid,&spid).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblemSparePart,
        version_number : 0,
        target_id : pid,
        other_target_id: Some(spid)
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}
