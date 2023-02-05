use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope, post};
use uuid::Uuid;

use crate::{AppState, repo::spare_part::{find_all_spare_parts, fetch_spare_part_by_id}};

pub fn scope() -> Scope{
  web::scope("/spare-part")
    .service(get_all)
    .service(get_spare_part_by_id)
}

#[get("/all")]
async fn get_all(state : Data<AppState>) -> impl Responder{
  HttpResponse::Ok().json(find_all_spare_parts(&state).await)
}


#[post("/part")]
async fn get_spare_part_by_id(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  HttpResponse::Ok().json(fetch_spare_part_by_id(&state,id.into_inner()).await)
}
