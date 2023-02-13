use actix_web::web::Data;
use sqlx::{error::Error,query_as,query};
use uuid::Uuid;

use crate::AppState;
use rec::{
  model::employee::Employee,
  crud_sync::{
    CudVersion,
    Cud,
    Table
  }
};

use super::syncing::record_version;

pub async fn find_all(state : &Data<AppState>) -> Result<Vec<Employee>,Error> {
    match query_as!(Employee,r#"
    select
      id,
      department_id,
      position,
      first_name,
      middle_name,
      last_name,
      card_id,
      password
    from employee where card_id <> 0
        "#).fetch_all(&state.db).await {
    Ok(employees) => Ok(employees),
    Err(err) => Err(err)
  }
}

pub async fn save(state : &Data<AppState>,employee : Employee) -> Result<(),Error> {
  let Employee{id,department_id,card_id,position,first_name,middle_name,last_name,password} = employee;
  let row = query!("
    INSERT INTO employee(
    id,
    department_id,
    position,
    first_name,
    middle_name,
    last_name,
    card_id,
    password)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8);",
                         id,
                         department_id,
                         position,
                         first_name,
                         middle_name,
                         last_name,
                         card_id,
                         password
  ).execute(&state.db);

  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
      cud : Cud::Create,
      target_table : Table::Employee,
      target_id : id,
      other_target_id : None,
      version_number : 0
  }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
      }
    },
    Err(err) => Err(err)
  }
}

pub async fn update(state : &Data<AppState>,employee : Employee) -> Result<(),Error> {
  let Employee{id,department_id,card_id,position,first_name,middle_name,last_name,password} = employee;
  let row = query!("
    UPDATE employee SET
    department_id = $2,
    position      = $3,
    first_name    = $4,
    middle_name   = $5,
    last_name     = $6,
    card_id       = $7,
    password      = $8
    WHERE id = $1;",
    id,
    department_id,
    position,
    first_name,
    middle_name,
    last_name,
    card_id,
    password
  ).execute(&state.db);

  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
      cud : Cud::Update,
      target_table : Table::Employee,
      target_id : id,
      other_target_id : None,
      version_number : 0
  }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
      }
    },
    Err(err) => Err(err)
  }
}

pub async fn delete(state : &Data<AppState>,id : Uuid) -> Result<(),Error> {
  let row = query!("
    DELETE FROM employee
    WHERE id = $1;",
    id
  ).execute(&state.db);

  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
      cud : Cud::Delete,
      target_table : Table::Employee,
      target_id : id,
      other_target_id : None,
      version_number : 0
  }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
      }
    },
    Err(err) => Err(err)
  }
}

pub async fn get_employee_by_card_id(state : &Data<AppState>,card_id : i16) -> Result<Employee,Error> {
  let row = query_as!(Employee,r#"select
      id,
      department_id,
      position,
      first_name,
      middle_name,
      last_name,
      card_id,
      password
 from employee where card_id = $1"#,card_id)
    .fetch_one(&state.db);
  match row.await {
    Ok(emp) => Ok(emp),
    Err(err) => Err(err)
  }
}

pub async fn fetch_employee_by_id(state : &Data<AppState>,id : Uuid) -> Result<Employee,Error> {
  let row = query_as!(Employee,r#"select
      id,
      department_id,
      position,
      first_name,
      middle_name,
      last_name,
      card_id,
      password
 from employee where id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(emp) => Ok(emp),
    Err(err) => Err(err)
  }
}

pub async fn get_employee_department_id_by_id(state : &Data<AppState>,id : Uuid) -> Result<Uuid,Error> {
  let row = query!(r#"
      select department_id from employee where id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(record) => Ok(record.department_id),
    Err(err) => Err(err)
  }
}
