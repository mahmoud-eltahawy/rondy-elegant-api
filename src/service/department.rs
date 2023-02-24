use actix_web::{
  web::{Data, self},
  Responder,
  HttpResponse,
  Scope,
  post,get,delete,put
};
use rec::{
  model::department::Department,
  crud_sync::{CudVersion,Cud,Table}
};
use uuid::Uuid;

use crate::{
  AppState,
  repo::{syncing::record_version, department::{fetch_department_by_id, save, update, delete}}};

pub fn scope() -> Scope{
  web::scope("/dep")
    .service(get_department_by_id)
    .service(save_department)
    .service(update_department)
    .service(delete_department)
}
#[get("/{id}")]
async fn get_department_by_id(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  match fetch_department_by_id(&state,id.into_inner()).await{
    Some(dep) => HttpResponse::Ok().json(dep),
    None      => HttpResponse::InternalServerError().into()
  }
}

#[delete("/{id}")]
async fn delete_department(state : Data<AppState>,id : web::Path<Uuid>) -> impl Responder{
  let id = id.into_inner();
  match delete(&state,&id).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::Department,
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
async fn save_department(state : Data<AppState>,dep : web::Json<Department>) -> impl Responder{
  let dep = dep.into_inner();
  let dep_id = dep.id;
  match save(&state,dep).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud                 : Cud::Create,
        target_table        : Table::Department,
        version_number      : 0,
        target_id           : dep_id,
        other_target_id     : None
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[put("/")]
async fn update_department(state : Data<AppState>,dep : web::Json<Department>) -> impl Responder{
  let dep = dep.into_inner();
  let dep_id = dep.id;
  match update(&state,dep).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud             : Cud::Update,
        target_table    : Table::Department,
        version_number  : 0,
        target_id       : dep_id,
        other_target_id : None
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}
