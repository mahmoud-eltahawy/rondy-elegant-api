use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope};

use crate::{AppState, repo::machine::find_all_machines};

pub fn scope() -> Scope{
  web::scope("/machine")
    .service(get_all)
}

#[get("/all")]
async fn get_all(state : Data<AppState>) -> impl Responder{
  HttpResponse::Ok().json(find_all_machines(state).await)
}
