use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope, post};
use rec::model::spare_part::SparePart;
use uuid::Uuid;

use crate::{AppState, repo::spare_part::{find_all_spare_parts, fetch_spare_part_by_id, save, update, delete}};

pub fn scope() -> Scope{
  web::scope("/spare-part")
    .service(get_all)
    .service(get_spare_part_by_id)
    .service(save_spare_part)
    .service(update_spare_part)
    .service(delete_spare_part)
}

#[get("/all")]
async fn get_all(state : Data<AppState>) -> impl Responder{
  match find_all_spare_parts(&state).await{
    Some(parts) => HttpResponse::Ok().json(parts),
    None        => HttpResponse::InternalServerError().into()
  }
}


#[post("/part")]
async fn get_spare_part_by_id(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  match fetch_spare_part_by_id(&state,id.into_inner()).await{
    Some(part) => HttpResponse::Ok().json(part),
    None       => HttpResponse::InternalServerError().into()
  }
}

#[post("/save")]
async fn save_spare_part(state : Data<AppState>,part : web::Json<SparePart>) -> impl Responder{
  match save(&state,part.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/update")]
async fn update_spare_part(state : Data<AppState>,part : web::Json<SparePart>) -> impl Responder{
  match update(&state,part.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/delete")]
async fn delete_spare_part(state : Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  match delete(&state,id.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}
