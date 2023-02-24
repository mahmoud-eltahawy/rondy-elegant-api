use std::cmp::Ordering;

use actix_web::{
    web::{Data, self}, Responder, HttpResponse, Scope, get};
use rec::crud_sync::CudVersion;

use crate::{AppState, repo::syncing::{get_version, last_version}};

pub fn scope() -> Scope{
  web::scope("/sync")
    .service(get_last_updates)
}


#[get("/{version}")]
async fn get_last_updates(state : Data<AppState>,version :web::Path<u64>) -> impl Responder{
  let version = version.into_inner();
  let current_version = match last_version(&state.db).await {
    Ok(v) => v,
    Err(_) => return HttpResponse::InternalServerError().into()
  };

  let mut versions : Vec<CudVersion> = Vec::new();

  match version.cmp(&current_version) {
    Ordering::Equal   => HttpResponse::Ok().json(versions),
    Ordering::Greater => HttpResponse::NotFound().into(),
    Ordering::Less    => {
      for v in version+1..=current_version{
        if let Ok(cv) = get_version(&state.db, v).await {
          versions.push(cv)
        }
      }
      HttpResponse::Ok().json(versions)
    }
  }
}
