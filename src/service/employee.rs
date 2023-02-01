use actix_web::{
    get,web, Responder, HttpResponse, Scope, post};
use bcrypt::BcryptResult;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{AppState, repo::{*, shift::{find_db_shift_by_date_and_order, save_db_shift}}, model::{employee::Employee, shift::DbShift}, timer::{get_current_date, get_relative_now, get_current_order}};

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

async fn get_or_save_db_shift(state : web::Data<AppState>) -> Option<DbShift>{
  let now = get_relative_now();
  let date = get_current_date(now);
  let order = get_current_order(now);
  if let Some(date) = date {
    match find_db_shift_by_date_and_order(state.clone(), date, order.clone()).await {
      Some(shift) => return Some(shift),
      None        =>{
        match save_db_shift(state, DbShift{
          id: Uuid::new_v4(),
          shift_date: date,
          shift_order: order as i16
        }).await {
          Some(shift) => return Some(shift),
          None        => return None
        }
      }
    }
  } else {
    None
  }
}



#[post("/login")]
async fn login(state : web::Data<AppState>,
               cred : web::Json<Credentials>) -> impl Responder{
  let Credentials{card_id,password} = cred.into_inner();
  let employee;
  match employee::get_employee_by_card_id(state.clone(), card_id).await {
    Ok(result) => employee = result,
    Err(err)   => return HttpResponse::NotFound().json(err.to_string())
  }
  match verify_password(password, &employee.password) {
    Ok(result) => if result {
        if let Some(shift) = get_or_save_db_shift(state).await {
          HttpResponse::Ok().json(Some((employee,shift.id)))
        } else {
          HttpResponse::NonAuthoritativeInformation().json(None::<(Employee,Uuid)>)
        }
      } else {
        HttpResponse::NonAuthoritativeInformation().json(None::<(Employee,Uuid)>)
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
