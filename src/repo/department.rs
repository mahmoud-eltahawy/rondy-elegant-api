use std::error::Error;

use actix_web::web::Data;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::model::department::Department;

pub async fn fetch_department_by_id(state : &Data<AppState>,id : Uuid) -> Option<Department> {
  let row = query_as!(Department,r#"
        select id,boss_id,department_id,name
        from department WHERE id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(dep) =>Some(dep),
    Err(_) => None
  }
}

pub async fn save(state : &Data<AppState>,department : Department) -> Result<(),Box<dyn Error>> {
  let Department{id,boss_id,department_id,name} = department;
  let row = query!("
    INSERT INTO department(id,boss_id,department_id,name)
    VALUES($1,$2,$3,$4);",
    id,boss_id,department_id,name).execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn update(state : &Data<AppState>,department : Department) -> Result<(),Box<dyn Error>> {
  let Department{id,boss_id,department_id,name} = department;
  let row = query!("
    UPDATE department SET
    boss_id         = $2,
    department_id   = $3,
    name            = $4
    WHERE id        = $1;",
    id,boss_id,department_id,name).execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn delete(state : &Data<AppState>,id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    DELETE FROM department
    WHERE id = $1;",
    id).execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
