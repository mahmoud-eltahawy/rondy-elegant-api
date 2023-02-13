use actix_web::{Scope, web::{self, Data}, Responder, HttpResponse, post};
use rec::model::note::DbNote;

use crate::{repo::relations::shift::{save_note_to_shift, remove_note_to_shift}, AppState};

pub fn scope() -> Scope{
  web::scope("/shift")
    .service(save_note)
    .service(delete_note)
}

#[post("/note-save")]
async fn save_note(state : Data<AppState>,note : web::Json<DbNote>) -> impl Responder{
  match save_note_to_shift(&state,note.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/note-delete")]
async fn delete_note(state : Data<AppState>,note : web::Json<DbNote>) -> impl Responder{
  match remove_note_to_shift(&state,note.into_inner()).await {
    Ok(_) => HttpResponse::Ok(),
    Err(_) => HttpResponse::InternalServerError()
  }
}
