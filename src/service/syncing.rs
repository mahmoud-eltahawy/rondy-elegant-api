use actix_web::{
    web::{Data, self}, Responder, HttpResponse, Scope, post};
use rec::crud_sync::CudVersion;

use crate::{AppState, repo::syncing::{get_version, last_version}};

pub fn scope() -> Scope{
  web::scope("/sync")
    .service(get_last_updates)
}


#[post("/update")]
async fn get_last_updates(state : Data<AppState>,version :web::Json<u64>) -> impl Responder{
  let version = version.into_inner();
  let current_version = match last_version(&state.db).await {
    Ok(v) => v,
    Err(_) => return HttpResponse::InternalServerError().into()
  };

  let mut versions : Vec<CudVersion> = Vec::new();

  if version == current_version{
    return HttpResponse::Ok().json(versions);
  } else if version > current_version {
    return HttpResponse::NotFound().into();
  }

  for v in version+1..=current_version{
    match get_version(&state.db, v).await{
        Ok(cv) => versions.push(cv),
        Err(_) => ()
    }
  }

  HttpResponse::Ok().json(versions)
}
