use actix_web::{
    get,
    web::Data, Responder, HttpResponse};

use crate::{AppState, repo::problem::find_all_probelms};

#[get("/test")]
pub async fn test(state : Data<AppState>) -> impl Responder{
  HttpResponse::Ok().json(find_all_probelms(state).await)
}
