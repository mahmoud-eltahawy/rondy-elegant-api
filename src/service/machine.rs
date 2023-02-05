use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope, post};
use uuid::Uuid;

use crate::{AppState, repo::machine::{find_all_machines, fetch_machine_by_id}};

pub fn scope() -> Scope{
  web::scope("/machine")
    .service(get_all)
    .service(get_machine_by_id)
}

#[get("/all")]
async fn get_all(state : Data<AppState>) -> impl Responder{
  HttpResponse::Ok().json(find_all_machines(&state).await)
}

#[post("/machine")]
async fn get_machine_by_id(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  HttpResponse::Ok().json(fetch_machine_by_id(&state,id.into_inner()).await)
}
