use actix_web::{
    get,web, Responder, HttpResponse, Scope, post};
use bcrypt::BcryptResult;
use serde::{Serialize, Deserialize};

use crate::{AppState, repo::*, model::employee::Employee};

pub fn scope() -> Scope{
  web::scope("/emp")
    .service(all)
    .service(save)
    .service(login)
}

#[get("/all")]
async fn all(state : web::Data<AppState>) -> impl Responder{
  match employee::find_all(state).await {
    Ok(result) => HttpResponse::Ok().json(result),
    Err(err)   => HttpResponse::NotFound().json(err.to_string())
  }
}

#[post("/save")]
async fn save(state : web::Data<AppState>, employee : web::Json<Employee>) -> impl Responder{
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

#[derive(Serialize,Deserialize)]
struct Credentials{
  card_id : i16,
  password: String
}

#[post("/login")]
async fn login(state : web::Data<AppState>,
               cred : web::Json<Credentials>) -> impl Responder{
  let Credentials{card_id,password} = cred.into_inner();
  let employee;
  match employee::get_employee_by_card_id(state, card_id).await {
    Ok(result) => employee = result,
    Err(err)   => return HttpResponse::NotFound().json(err.to_string())
  }
  match verify_password(password, &employee.password) {
    Ok(result) => if result {
        HttpResponse::Ok().json(Some(employee))
      } else {
        HttpResponse::NonAuthoritativeInformation().json(None::<Employee>)
      },
    Err(err)   => return HttpResponse::NotFound().json(err.to_string())
  }
}

fn  hash_password(password : String) -> BcryptResult<String>{
  bcrypt::hash(password, 8)
}

fn  verify_password(password : String,hash : &str) -> BcryptResult<bool>{
  bcrypt::verify(password, hash)
}
