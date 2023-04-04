use actix_web::{
    post,get,put,delete,web::{Data, self}, Responder, HttpResponse, Scope};
use uuid::Uuid;

use crate::{
  AppState,
  repo::{problem::{
    fetch_problem_by_id,
    save, update, delete
  }, syncing::record_version},
};
use rec::{
  model::problem::Problem,
  crud_sync::{
    CudVersion,
    Cud,
    Table
  }
};

pub fn scope() -> Scope{
  web::scope("/problem")
    .service(get_problem_by_id)
    .service(save_problem)
    .service(update_problem)
    .service(delete_problem)
}

#[get("/{id}")]
async fn get_problem_by_id(state : Data<AppState>,id : web::Path<Uuid>) -> impl Responder{
  match fetch_problem_by_id(&state,id.into_inner()).await{
    Some(problem) => HttpResponse::Ok().json(problem),
    None          => HttpResponse::InternalServerError().into()
  }
}

#[delete("/{id}")]
async fn delete_problem(state : Data<AppState>,id : web::Path<Uuid>) -> impl Responder{
  let id = id.into_inner();
  match delete(&state,&id).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::Problem,
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
async fn save_problem(state : Data<AppState>,problem : web::Json<Problem<Uuid>>) -> impl Responder{
  let problem = problem.into_inner();
  match save(&state,&problem).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::Problem,
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
async fn update_problem(state : Data<AppState>,problem : web::Json<Problem<Uuid>>) -> impl Responder{
  let problem = problem.into_inner();
  match update(&state,&problem).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Update,
        target_table : Table::Problem,
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
