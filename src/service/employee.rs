use actix_web::{
    get,web, Responder, HttpResponse, Scope, post};
use bcrypt::BcryptResult;
use uuid::Uuid;

use crate::{AppState,
            repo::{
              shift::get_or_save_db_shift,
              employee::{
                fetch_employee_by_id,
                find_all, save, get_employee_by_card_id, update, delete
              }},
};
use rec::model::employee::{Employee, Cred};

pub fn scope() -> Scope{
  web::scope("/emp")
    .service(all)
    .service(save_employee)
    .service(update_employee)
    .service(delete_employee)
    .service(get_employee_by_id)
    .service(login)
}

#[get("/all")]
async fn all(state : web::Data<AppState>) -> impl Responder{
  match find_all(&state).await {
    Ok(result) => HttpResponse::Ok().json(result),
    Err(_)   => HttpResponse::InternalServerError().into()
  }
}

#[post("/save")]
async fn save_employee(state : web::Data<AppState>, employee : web::Json<Employee>) -> impl Responder{
  let employee = match hash_employee(employee.into_inner()) {
    Ok(employee) => employee,
    Err(_)    => return HttpResponse::InternalServerError().into()
  };

  match save(&state, employee).await {
    Ok(_)    => HttpResponse::Ok(),
    Err(_)   => HttpResponse::InternalServerError()
  }
}

#[post("/update")]
async fn update_employee(state : web::Data<AppState>, employee : web::Json<Employee>) -> impl Responder{
  let employee = match hash_employee(employee.into_inner()) {
    Ok(employee) => employee,
    Err(_)    => return HttpResponse::InternalServerError()
  };
  match update(&state, employee).await {
    Ok(_)    => HttpResponse::Ok(),
    Err(_)   => HttpResponse::InternalServerError()
  }
}

#[post("/delete")]
async fn delete_employee(state : web::Data<AppState>, id : web::Json<Uuid>) -> impl Responder{
  match delete(&state, id.into_inner()).await {
    Ok(_)    => HttpResponse::Ok(),
    Err(_)   => HttpResponse::InternalServerError()
  }
}

#[post("/emp")]
async fn get_employee_by_id(state : web::Data<AppState>,
               id : web::Json<Uuid>) -> impl Responder{
  match fetch_employee_by_id(&state, id.into_inner()).await {
    Ok(result) => HttpResponse::Ok().json(result),
    Err(_)     => HttpResponse::InternalServerError().into()
  }
}

#[post("/login")]
async fn login(state : web::Data<AppState>,
               cred : web::Json<Cred>) -> impl Responder{
  let Cred{card_id,password} = cred.into_inner();
  let employee;
  match get_employee_by_card_id(&state, card_id).await {
    Ok(result) => employee = result,
    Err(_)   => return HttpResponse::InternalServerError().into()
  }
  match verify_password(password, &employee.password) {
    Ok(result) => if result {
        if let Some(shift) = get_or_save_db_shift(&state).await {
          HttpResponse::Ok().json((employee,shift.id))
        } else {
          HttpResponse::InternalServerError().into()
        }
      } else {
        HttpResponse::NonAuthoritativeInformation().into()
      },
    Err(_)   => return HttpResponse::InternalServerError().into()
  }
}

fn  hash_password(password : String) -> BcryptResult<String>{
  bcrypt::hash(password, 8)
}

fn  verify_password(password : String,hash : &str) -> BcryptResult<bool>{
  bcrypt::verify(password, hash)
}

fn hash_employee(employee : Employee) -> Result<Employee,String>{
  let Employee { id,
                 department_id,
                 position,
                 first_name,
                 middle_name,
                 last_name,
                 card_id,
                 password
  } = employee;
  match hash_password(password){
    Ok(password) =>Ok(Employee {
      id,
      department_id,
      position,
      first_name,
      middle_name,
      last_name,
      card_id,
      password
    }),
    Err(err)    => Err(err.to_string())
  }
}
