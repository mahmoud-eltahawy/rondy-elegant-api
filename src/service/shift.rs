use actix_web::{Scope, web::{self, Data}, post, Responder, HttpResponse};
use uuid::Uuid;
use crate::{
  AppState,
  repo::shift::{
    find_db_shift_by_id,
    remove_db_shift,
    get_or_save_db_shift
  }
};

pub fn scope() -> Scope{
  web::scope("/shift")
    .service(save_shift)
    .service(delete_shift)
    .service(get_shift_by_id)
}

#[post("/save-or")]
async fn save_shift(state : web::Data<AppState>) -> impl Responder{
  HttpResponse::Ok().json(get_or_save_db_shift(&state).await)
}

#[post("/delete")]
async fn delete_shift(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  match remove_db_shift(&state,id.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/shift")]
async fn get_shift_by_id(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  match find_db_shift_by_id(&state,id.into_inner()).await{
    Some(shift) => HttpResponse::Ok().json(shift),
    None        => HttpResponse::InternalServerError().into()
  }
}
