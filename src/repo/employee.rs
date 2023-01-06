
use actix_web::web::Data;
use sqlx::error::Error;
use uuid::Uuid;

use crate::{AppState, model::employee::Employee};

pub async fn find_all(state : Data<AppState>) -> Result<Vec<Employee>,Error> {
    let query = "
        select
          id,
          department_id,
          position,
          first_name,
          middle_name,
          last_name,
          card_id,
          password
       from employee
            ";
        match sqlx::query_as::<_, Employee>(query).fetch_all(&state.db).await {
        Ok(employees) => Ok(employees),
        Err(err) => Err(err)
    }
}

pub async fn save(state : Data<AppState>,employee : Employee) -> Result<Uuid,Error> {
  let Employee{id:_,department_id,card_id,position,first_name,middle_name,last_name,password} = employee;
  let id = Uuid::new_v4();
  let row = sqlx::query("
    INSERT INTO employee(
    id,
    department_id,
    position,
    first_name,
    middle_name,
    last_name,
    card_id,
    password)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8);")
      .bind(id)
      .bind(department_id)
      .bind(position)
      .bind(first_name)
      .bind(middle_name)
      .bind(last_name)
      .bind(card_id)
      .bind(password)
      .execute(&state.db);
    match row.await {
      Ok(_) => Ok(id),
      Err(err) => Err(err)
    }
}
