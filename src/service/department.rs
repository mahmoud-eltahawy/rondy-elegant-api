use actix_web::{
  web::{Data, self},
  Responder,
  HttpResponse,
  Scope,
  post,get,delete,put
};
use rec::{
  model::{department::Department, permissions::Permissions},
  crud_sync::{CudVersion,Cud,Table}
};
use uuid::Uuid;

use crate::{
  AppState,
  repo::{syncing::record_version, department::{fetch_department_by_id, save, update, delete, set_department_boss, fetch_department_boss_id_by_id}, employee::fetch_employee_department_id_by_id, permissions::{fetch_permissions_by_id, self}}};

pub fn scope() -> Scope{
  web::scope("/dep")
    .service(get_department_by_id)
    .service(save_department)
    .service(update_department)
    .service(delete_department)
    .service(set_department_boss_by_id)
}
#[get("/{id}")]
async fn get_department_by_id(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  match fetch_department_by_id(&state,id.into_inner()).await{
    Some(dep) => HttpResponse::Ok().json(dep),
    None      => HttpResponse::InternalServerError().into()
  }
}

#[get("/{id}/boss")]
async fn set_department_boss_by_id(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  let employee_id = id.into_inner();
  let Ok(department_id) = fetch_employee_department_id_by_id(&state, &employee_id).await else {
    return HttpResponse::InternalServerError().into()
  };
  let Ok(some_boss_id) = fetch_department_boss_id_by_id(&state, &department_id).await else{
    return HttpResponse::InternalServerError().into()
  };
  if let Some(boss_id) = some_boss_id {
    let Some(old_boss_permissions) = fetch_permissions_by_id(&state, boss_id).await else{
      return HttpResponse::InternalServerError().into()
    };
    let Ok(_) = permissions::update(&state, Permissions::default(old_boss_permissions.id)).await else{
      return HttpResponse::InternalServerError().into()
    };
    let Ok(_) = record_version(&state, CudVersion{
        cud : Cud::Update,
        target_table : Table::Permissions,
        version_number : 0,
        target_id : old_boss_permissions.id,
        other_target_id: None
      }).await else{
      return HttpResponse::InternalServerError().into()
    };
    let Ok(_) = permissions::update(&state, Permissions{id : employee_id,..old_boss_permissions}).await else{
      return HttpResponse::InternalServerError().into()
    };
    let Ok(_) = record_version(&state, CudVersion{
        cud : Cud::Update,
        target_table : Table::Permissions,
        version_number : 0,
        target_id : employee_id,
        other_target_id: None
      }).await else{
      return HttpResponse::InternalServerError().into()
    };
  }
  match set_department_boss(&state,&employee_id,&department_id).await{
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Update,
        target_table : Table::Department,
        version_number : 0,
        target_id : department_id,
        other_target_id: None
      }).await {
        Ok(_)  => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError().into()
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
async fn save_department(state : Data<AppState>,dep : web::Json<Department<Uuid>>) -> impl Responder{
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
async fn update_department(state : Data<AppState>,dep : web::Json<Department<Uuid>>) -> impl Responder{
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
