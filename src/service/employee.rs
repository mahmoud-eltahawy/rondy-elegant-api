use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope, post};

use crate::{AppState, repo::*, model::employee::Employee};

pub fn scope() -> Scope{
  web::scope("/emp")
    .service(all)
    .service(save)
}

#[get("/all")]
async fn all(state : Data<AppState>) -> impl Responder{
  match employee::find_all(state).await {
    Ok(result) => HttpResponse::Ok().json(result),
    Err(err)   => HttpResponse::NotFound().json(err.to_string())
  }
}


#[post("/save")]
async fn save(state : Data<AppState>, employee : web::Json<Employee>) -> impl Responder{
  match employee::save(state, employee.into_inner()).await {
    Ok(emp)    => HttpResponse::Ok().json(emp),
    Err(err)   => HttpResponse::NotFound().json(err.to_string())
  }
}
