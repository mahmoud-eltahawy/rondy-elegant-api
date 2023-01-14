use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope};

use crate::{AppState, repo::problem::find_all_probelms};

pub fn scope() -> Scope{
  web::scope("/problem")
    .service(get_all)
}

#[get("/all")]
async fn get_all(state : Data<AppState>) -> impl Responder{
  HttpResponse::Ok().json(find_all_probelms(state).await)
}
