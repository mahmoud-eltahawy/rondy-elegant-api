use actix_web::{Scope, web::{self, Data}, post, Responder, HttpResponse};
use uuid::Uuid;
use rec::model::shift::DateOrder;
use crate::{AppState, repo::shift::{find_db_shift_by_id, remove_db_shift, find_db_shift_by_date_and_order}};

pub fn scope() -> Scope{
  web::scope("/shift")
    .service(delete_shift)
    .service(get_shift_by_id)
    .service(get_shift_by)
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

#[post("/shift-by")]
async fn get_shift_by(state : Data<AppState>,od :web::Json<DateOrder>) -> impl Responder{
  match find_db_shift_by_date_and_order(&state,od.into_inner()).await{
    Some(shift) => HttpResponse::Ok().json(shift),
    None        => HttpResponse::InternalServerError().into()
  }
}
