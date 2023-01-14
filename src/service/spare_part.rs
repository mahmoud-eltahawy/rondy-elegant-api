use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope};

use crate::{AppState, repo::spare_part::find_all_spare_parts};

pub fn scope() -> Scope{
  web::scope("/spare-part")
    .service(get_all)
}

#[get("/all")]
async fn get_all(state : Data<AppState>) -> impl Responder{
  HttpResponse::Ok().json(find_all_spare_parts(state).await)
}
