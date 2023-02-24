use actix_web::{web, Responder, HttpResponse, Scope, post,put,delete,get};
use bcrypt::BcryptResult;
use uuid::Uuid;

use crate::{
  AppState,
  repo::{employee::{
    fetch_employee_by_id,
    save,
    update,
    delete
  }, syncing::record_version}
};
use rec::{
  model::employee::Employee,
  crud_sync::{
    CudVersion,
    Cud,
    Table
  }
};

pub fn scope() -> Scope{
  web::scope("/emp")
    .service(save_employee)
    .service(update_employee)
    .service(delete_employee)
    .service(get_employee_by_id)
}

#[get("/{id}")]
async fn get_employee_by_id(state : web::Data<AppState>,id : web::Path<Uuid>) -> impl Responder{
  match fetch_employee_by_id(&state, id.into_inner()).await {
    Ok(result) => HttpResponse::Ok().json(result),
    Err(_)     => HttpResponse::InternalServerError().into()
  }
}

#[delete("/{id}")]
async fn delete_employee(state : web::Data<AppState>, id : web::Path<Uuid>) -> impl Responder{
  let id = id.into_inner();
  match delete(&state, &id).await {
    Ok(_)    => {
      match record_version(&state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::Employee,
        target_id : id,
        other_target_id : None,
        version_number : 0
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_)   => HttpResponse::InternalServerError()
  }
}

#[post("/")]
async fn save_employee(state : web::Data<AppState>, employee : web::Json<Employee>) -> impl Responder{
  let employee = match hash_employee(employee.into_inner()) {
    Ok(employee) => employee,
    Err(_)    => return HttpResponse::InternalServerError()
  };

  match save(&state, &employee).await {
    Ok(_)    =>{
      match record_version(&state, CudVersion{
        cud : Cud::Create,
        target_table : Table::Employee,
        target_id : employee.id,
        other_target_id : None,
        version_number : 0
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    },
    Err(_)   => HttpResponse::InternalServerError()
  }
}

#[put("/")]
async fn update_employee(state : web::Data<AppState>, employee : web::Json<Employee>) -> impl Responder{
  let employee = match hash_employee(employee.into_inner()) {
    Ok(employee) => employee,
    Err(_)    => return HttpResponse::InternalServerError()
  };
  match update(&state, &employee).await {
    Ok(_)    =>{
      match record_version(&state, CudVersion{
        cud : Cud::Update,
        target_table : Table::Employee,
        target_id : employee.id,
        other_target_id : None,
        version_number : 0
      }).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
      }
    } ,
    Err(_)   => HttpResponse::InternalServerError()
  }
}

fn  hash_password(password : String) -> BcryptResult<String>{
  bcrypt::hash(password, 8)
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
