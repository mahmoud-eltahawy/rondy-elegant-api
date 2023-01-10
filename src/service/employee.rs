use actix_web::{
    get,
    web::{Data, self}, Responder, HttpResponse, Scope, post};
use bcrypt::BcryptResult;

use crate::{AppState, repo::*, model::employee::Employee};

pub fn scope() -> Scope{
  web::scope("/emp")
    .service(all)
    .service(save)
}

#[get("/all")]
async fn all(state : Data<AppState>) -> impl Responder{
  match employee::find_all(state).await {
    Ok(result) => HttpResponse::Ok().json(result),
    Err(err)   => HttpResponse::NotFound().json(err.to_string())
  }
}


#[post("/save")]
async fn save(state : Data<AppState>, employee : web::Json<Employee>) -> impl Responder{
  let mut employee = employee.into_inner();
  match hash_password(employee.password){
    Ok(hashing) => employee.password = hashing,
    Err(err)    => return HttpResponse::NotFound().json(err.to_string())
  };
  match employee::save(state, employee).await {
    Ok(emp)    => HttpResponse::Ok().json(emp),
    Err(err)   => HttpResponse::NotFound().json(err.to_string())
  }
}


fn  hash_password(password : String) -> BcryptResult<String>{
  bcrypt::hash(password, 8)
}

fn  verify_password(password : String,hash : &str) -> BcryptResult<bool>{
  bcrypt::verify(password, hash)
}
