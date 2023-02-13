use actix_web::{
    web::{Data, self}, Responder, HttpResponse, Scope, post,get};
use rec::model::machine::Machine;
use uuid::Uuid;

use crate::{AppState, repo::machine::{fetch_machine_by_id,fetch_all, save, update, delete}};

pub fn scope() -> Scope{
  web::scope("/machine")
    .service(all)
    .service(get_machine_by_id)
    .service(save_machine)
    .service(update_machine)
    .service(delete_machine)
}

#[get("/all")]
async fn all(state : web::Data<AppState>) -> impl Responder{
  match fetch_all(&state).await {
    Some(result) => HttpResponse::Ok().json(result),
    None   => HttpResponse::InternalServerError().into()
  }
}

#[post("/save")]
async fn save_machine(state : Data<AppState>,machine :web::Json<Machine>) -> impl Responder{
  match save(&state,machine.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/update")]
async fn update_machine(state : Data<AppState>,machine :web::Json<Machine>) -> impl Responder{
  match update(&state,machine.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/delete")]
async fn delete_machine(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  match delete(&state,id.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/machine")]
async fn get_machine_by_id(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  match fetch_machine_by_id(&state,id.into_inner()).await{
    Some(machine) => HttpResponse::Ok().json(machine),
    None          => HttpResponse::InternalServerError().into()
  }
}
