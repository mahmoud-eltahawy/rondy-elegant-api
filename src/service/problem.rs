use actix_web::{
    get, post,
    web::{Data, self}, Responder, HttpResponse, Scope};
use uuid::Uuid;

use crate::{
  AppState,
  repo::problem::{
    fetch_problem_by_id,
    save, fetch_problem_by_department_id, update, delete
  },
};
use rec::model::problem::Probelm;

pub fn scope() -> Scope{
  web::scope("/problem")
    .service(get_by_department_id)
    .service(get_problem_by_id)
    .service(save_problem)
    .service(update_problem)
    .service(delete_problem)
}

#[get("/all")]
async fn get_by_department_id(state : Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  match fetch_problem_by_department_id(&state,id.into_inner()).await{
    Some(problems) => HttpResponse::Ok().json(problems),
    None           => HttpResponse::InternalServerError().into()
  }
}

#[post("/problem")]
async fn get_problem_by_id(state : Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  match fetch_problem_by_id(&state,id.into_inner()).await{
    Some(problem) => HttpResponse::Ok().json(problem),
    None          => HttpResponse::InternalServerError().into()
  }
}

#[post("/save")]
async fn save_problem(state : Data<AppState>,problem : web::Json<Probelm>) -> impl Responder{
  match save(&state,problem.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/update")]
async fn update_problem(state : Data<AppState>,problem : web::Json<Probelm>) -> impl Responder{
  match update(&state,problem.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/delete")]
async fn delete_problem(state : Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  match delete(&state,id.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}
