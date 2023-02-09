use actix_web::{
    get, post,
    web::{Data, self}, Responder, HttpResponse, Scope};
use uuid::Uuid;

use crate::{
  AppState,
  repo::problem::{
    fetch_problem_by_id,
    save, fetch_problem_by_department_id
  },
};
use rec::model::problem::Probelm;

pub fn scope() -> Scope{
  web::scope("/problem")
    .service(get_by_department_id)
    .service(get_problem_by_id)
    .service(save_problem)
}

#[get("/all")]
async fn get_by_department_id(state : Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  HttpResponse::Ok().json(fetch_problem_by_department_id(&state,id.into_inner()).await)
}

#[post("/problem")]
async fn get_problem_by_id(state : Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  HttpResponse::Ok().json(fetch_problem_by_id(&state,id.into_inner()).await)
}

#[post("/save")]
async fn save_problem(state : Data<AppState>,problem : web::Json<Probelm>) -> impl Responder{
  match save(&state,problem.into_inner()).await {
    Ok(_) => HttpResponse::Ok().json(true),
    Err(_) => HttpResponse::Ok().json(false)
  }
}
