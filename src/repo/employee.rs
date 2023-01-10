use actix_web::web::Data;
use sqlx::{error::Error,query_as,query};
use uuid::Uuid;

use crate::{AppState, model::employee::Employee};

pub async fn find_all(state : Data<AppState>) -> Result<Vec<Employee>,Error> {
    match query_as!(Employee,r#"
    select
      id as "id?",
      department_id,
      position,
      first_name,
      middle_name,
      last_name,
      card_id,
      password
    from employee
        "#).fetch_all(&state.db).await {
    Ok(employees) => Ok(employees),
    Err(err) => Err(err)
  }
}

pub async fn save(state : Data<AppState>,employee : Employee) -> Result<Uuid,Error> {
  let Employee{id:_,department_id,card_id,position,first_name,middle_name,last_name,password} = employee;
  let id = Uuid::new_v4();
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
    Ok(_) => Ok(id),
    Err(err) => Err(err)
  }
}

pub async fn get_employee_by_card_id(state : Data<AppState>,card_id : i16) -> Result<Employee,Error> {
  let row = query_as!(Employee,r#"select
      id as "id?",
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
