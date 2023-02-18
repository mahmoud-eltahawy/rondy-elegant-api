use actix_web::{
    web::{Data, self}, Responder, HttpResponse, Scope, post};
use rec::{
  model::machine::Machine,
  crud_sync::{
    CudVersion,
    Table,
    Cud
  }
};
use uuid::Uuid;

use crate::{AppState, repo::{machine::{fetch_machine_by_id, save, update, delete}, syncing::record_version}};

pub fn scope() -> Scope{
  web::scope("/machine")
    .service(get_machine_by_id)
    .service(save_machine)
    .service(update_machine)
    .service(delete_machine)
}

#[post("/machine")]
async fn get_machine_by_id(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  match fetch_machine_by_id(&state,id.into_inner()).await{
    Some(machine) => HttpResponse::Ok().json(machine),
    None          => HttpResponse::InternalServerError().into()
  }
}

#[post("/save")]
async fn save_machine(state : Data<AppState>,machine :web::Json<Machine>) -> impl Responder{
  let machine = machine.into_inner();
  match save(&state,&machine).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::Machine,
        target_id : machine.id,
        other_target_id : None,
        version_number : 0
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/update")]
async fn update_machine(state : Data<AppState>,machine :web::Json<Machine>) -> impl Responder{
  let machine = machine.into_inner();
  match update(&state,&machine).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Update,
        target_table : Table::Machine,
        target_id : machine.id,
        other_target_id : None,
        version_number : 0
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/delete")]
async fn delete_machine(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  let id = id.into_inner();
  match delete(&state,&id).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::Machine,
        target_id : id,
        other_target_id : None,
        version_number : 0
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}
