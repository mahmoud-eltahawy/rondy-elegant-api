use actix_web::web::Data;
use sqlx::{error::Error,query_as,query};
use uuid::Uuid;

use crate::AppState;
use rec::model::employee::Employee;

pub async fn save(state : &Data<AppState>,employee : &Employee) -> Result<(),Error> {
  let Employee{id,department_id,card_id,position,first_name,middle_name,last_name,password} = employee;
  let row = query!("
    INSERT INTO employee(
    id,department_id,position,
    first_name,middle_name,last_name,
    card_id,password)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8);",
    id,department_id,position,
    first_name,middle_name,last_name,
    card_id,password
  ).execute(&state.db);

  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn update(state : &Data<AppState>,employee : &Employee) -> Result<(),Error> {
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
    id,department_id,position,
    first_name,middle_name,last_name,
    card_id,password
  ).execute(&state.db);

  match row.await {
    Ok(_) =>Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn down(state : &Data<AppState>,id : Uuid) -> Result<(),Error> {
  let row = query!("
    UPDATE employee SET
    position = 'USER'
    WHERE id = $1;",id).execute(&state.db);
  match row.await {
    Ok(_) =>Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn up(state : &Data<AppState>,id : Uuid) -> Result<(),Error> {
  let row = query!("
    UPDATE employee SET
    position = 'SUPER_USER'
    WHERE id = $1;",id).execute(&state.db);
  match row.await {
    Ok(_) =>Ok(()),
    Err(err) => Err(err)
  }
}

pub async fn delete(state : &Data<AppState>,id : &Uuid) -> Result<(),Error> {
  let row = query!("
    DELETE FROM employee
    WHERE id = $1;",
    id
  ).execute(&state.db);

  match row.await {
    Ok(_) =>Ok(()),
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

pub async fn fetch_employee_department_id_by_id(state : &Data<AppState>,id : &Uuid) -> Result<Uuid,Error> {
  let row = query!(r#"
    select department_id
    from employee where id = $1"#,id)
  .fetch_one(&state.db);
  match row.await {
    Ok(emp) => Ok(emp.department_id),
    Err(err) => Err(err)
  }
}
