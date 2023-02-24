use actix_web::{Scope, web::{self, Data}, post,get,delete, Responder, HttpResponse};
use uuid::Uuid;
use crate::{
  AppState, repo::department_shift::{save_department_shift, remove_department_shift, find_shift_by_id, find_department_shift_by_id}
};

pub fn scope() -> Scope{
  web::scope("/shift")
    .service(save_shift)
    .service(delete_shift)
    .service(get_shift_by_id)
    .service(get_department_shift_by_id)
}

#[get("/{id}")]
async fn get_shift_by_id(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  match find_shift_by_id(&state,id.into_inner()).await{
    Ok(Some(shift)) => HttpResponse::Ok().json(shift),
    _               => HttpResponse::InternalServerError().into()
  }
}

#[get("/dep/{id}")]
async fn get_department_shift_by_id(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  match find_department_shift_by_id(&state,id.into_inner()).await{
    Ok(shift) => HttpResponse::Ok().json(shift),
    Err(_)    => HttpResponse::InternalServerError().into()
  }
}

#[delete("/{id}")]
async fn delete_shift(state : Data<AppState>,id :web::Path<Uuid>) -> impl Responder{
  match remove_department_shift(&state,id.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_)  => HttpResponse::InternalServerError()
  }
}

#[post("/{id}")]
async fn save_shift(state : web::Data<AppState>,id : web::Path<Uuid>) -> impl Responder{
  match save_department_shift(&state, id.into_inner()).await {
    Ok(_)  => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}
