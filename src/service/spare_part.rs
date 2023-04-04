use actix_web::{
  web::{Data, self},
  Responder,
  HttpResponse,
  Scope,
  post,get,put,delete
};
use rec::{
  model::spare_part::SparePart,
  crud_sync::{
    CudVersion,Cud,Table
  }
};
use uuid::Uuid;

use crate::{
  AppState,
  repo::{
    spare_part::{
      fetch_spare_part_by_id,
      save, update, delete
    }, syncing::record_version}};

pub fn scope() -> Scope{
  web::scope("/part")
    .service(get_spare_part_by_id)
    .service(save_spare_part)
    .service(update_spare_part)
    .service(delete_spare_part)
}
#[get("/{id}")]
async fn get_spare_part_by_id(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  match fetch_spare_part_by_id(&state,id.into_inner()).await{
    Some(part) => HttpResponse::Ok().json(part),
    None       => HttpResponse::InternalServerError().into()
  }
}

#[delete("/{id}")]
async fn delete_spare_part(state : Data<AppState>,id : web::Path<Uuid>) -> impl Responder{
  let id = id.into_inner();
  match delete(&state,&id).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::SparePart,
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
async fn save_spare_part(state : Data<AppState>,part : web::Json<SparePart<Uuid>>) -> impl Responder{
  let part = part.into_inner();
  match save(&state,&part).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::SparePart,
        version_number : 0,
        target_id : part.id,
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
async fn update_spare_part(state : Data<AppState>,part : web::Path<SparePart<Uuid>>) -> impl Responder{
  let part = part.into_inner();
  match update(&state,&part).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud             : Cud::Update,
        target_table    : Table::SparePart,
        version_number  : 0,
        target_id       : part.id,
        other_target_id : None
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}
