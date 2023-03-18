use actix_web::{
  web::{Data, self},
  Responder,
  HttpResponse,
  Scope,
  post,get,put
};
use rec::{
  model::permissions::Permissions,
  crud_sync::{CudVersion,Cud,Table}
};
use uuid::Uuid;

use crate::{
  AppState,
  repo::{
    syncing::record_version,
    permissions::{
      fetch_permissions_by_id,
      save,
      update, allow_permission, forbid_permission
    }
  }
};

pub fn scope() -> Scope{
  web::scope("/per")
    .service(get_permissions_by_id)
    .service(save_permissions)
    .service(update_permissions)
    .service(allow_permission_service)
    .service(forbid_permission_service)
}

#[get("/{id}")]
async fn get_permissions_by_id(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  match fetch_permissions_by_id(&state,id.into_inner()).await{
    Some(dep) => HttpResponse::Ok().json(dep),
    None      => HttpResponse::InternalServerError().into()
  }
}

#[get("/{id}/{permission}/allow")]
async fn allow_permission_service(state : Data<AppState>,
            path : web::Path<(Uuid,String)>) -> impl Responder{
  let (id,permission) = path.into_inner();
  match allow_permission(&state,&id,permission).await{
    Ok(_) => {
      match record_version(&state, CudVersion {
        cud             : Cud::Update,
        target_table    : Table::Permissions,
        version_number  : 0,
        target_id       : id,
        other_target_id : None
      }).await {
        Ok(_)  => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[get("/{id}/{permission}/forbid")]
async fn forbid_permission_service(state : Data<AppState>,
            path : web::Path<(Uuid,String)>) -> impl Responder{
  let (id,permission) = path.into_inner();
  match forbid_permission(&state,&id,permission).await {
    Ok(_)   => {
      match record_version(&state, CudVersion {
        cud             : Cud::Update,
        target_table    : Table::Permissions,
        version_number  : 0,
        target_id       : id,
        other_target_id : None
      }).await {
        Ok(_)  => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/")]
async fn save_permissions(state : Data<AppState>,permissions : web::Json<Permissions>) -> impl Responder{
  let permissions = permissions.into_inner();
  let permissions_id = permissions.id;
  match save(&state,permissions).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud                 : Cud::Create,
        target_table        : Table::Permissions,
        version_number      : 0,
        target_id           : permissions_id,
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
async fn update_permissions(state : Data<AppState>,permissions : web::Json<Permissions>) -> impl Responder{
  let permissions    = permissions.into_inner();
  let permissions_id = permissions.id;
  match update(&state,permissions).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud             : Cud::Update,
        target_table    : Table::Permissions,
        version_number  : 0,
        target_id       : permissions_id,
        other_target_id : None
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}
