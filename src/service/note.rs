use actix_web::{
  web::{self, Data},
  Scope, post, HttpResponse, Responder,
};
use rec::{crud_sync::{CudVersion,Table,Cud}, model::note::{DbNote, Note}};
use uuid::Uuid;

use crate::{
  repo::{
    syncing::record_version,
    note::{
      save_note_to_shift_problem,
      update_note,
      remove_note,
      save_note_to_shift,
      fetch_note_by_id
    }
  },
  AppState
};

pub fn scope() -> Scope{
  web::scope("/note")
    .service(get_note_by_id)
    .service(note_to_problem_save)
    .service(note_to_shift_save)
    .service(update)
    .service(delete)
}

#[post("/note")]
async fn get_note_by_id(state : Data<AppState>,id :web::Json<Uuid>) -> impl Responder{
  match fetch_note_by_id(&state,&id.into_inner()).await{
    Some(note) => HttpResponse::Ok().json(note),
    None       => HttpResponse::InternalServerError().into()
  }
}

#[post("/p-save")]
async fn note_to_problem_save(state : Data<AppState>,note : web::Json<DbNote>) -> impl Responder{
  let note = note.into_inner();
  match save_note_to_shift_problem(&state,&note).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblemNote,
        version_number : 0,
        target_id : note.id,
        other_target_id: note.shift_problem_id
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/s-save")]
async fn note_to_shift_save(state : Data<AppState>,note : web::Json<DbNote>) -> impl Responder{
  let note = note.into_inner();
  match save_note_to_shift(&state,&note).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftNote,
        version_number : 0,
        target_id : note.id,
        other_target_id: note.shift_problem_id
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/update")]
async fn update(state : Data<AppState>,note : web::Json<Note>) -> impl Responder{
  let note = note.into_inner();
  match update_note(&state,&note).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Update,
        target_table : Table::ShiftNote,
        version_number : 0,
        target_id : note.id,
        other_target_id: None
      }).await {
        Ok(_)    => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}

#[post("/delete")]
async fn delete(state : Data<AppState>,id : web::Json<Uuid>) -> impl Responder{
  let id = id.into_inner();
  match remove_note(&state,&id).await {
    Ok(_) => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftNote,
        version_number : 0,
        target_id : id,
        other_target_id: None
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_) => HttpResponse::InternalServerError()
  }
}
