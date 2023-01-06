use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope};

use crate::{AppState, repo::problem::find_all_probelms};

pub fn scope() -> Scope{
  web::scope("/test")
    .service(test1)
    .service(test2)
}

#[get("/test1")]
async fn test1(state : Data<AppState>) -> impl Responder{
  HttpResponse::Ok().json(find_all_probelms(state).await)
}

#[get("/test2")]
async fn test2() -> impl Responder{
  HttpResponse::Ok().json("test 2")
}
