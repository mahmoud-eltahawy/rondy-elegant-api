use std::error::Error;

use actix_web::web::Data;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::model::permissions::Permissions;

pub async fn fetch_permissions_by_id(state : &Data<AppState>,id : Uuid) -> Option<Permissions> {
  let row = query_as!(Permissions,r#"
        select * from permissions WHERE id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(dep) =>Some(dep),
    Err(_) => None
  }
}

pub async fn allow_permission(state : &Data<AppState>,
               id : &Uuid,permission : String) -> Result<(),Box<dyn Error>> {
  let sql = format!("UPDATE permissions SET {} = true WHERE id = $1",permission);
  let row = query(&sql)
    .bind(id)
    .execute(&state.db);
  match row.await {
    Ok(_)  => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn forbid_permission(state : &Data<AppState>,
               id : &Uuid,permission : String) -> Result<(),Box<dyn Error>> {
  let sql = format!("UPDATE permissions SET {} = false WHERE id = $1",permission);
  let row = query(&sql)
    .bind(id)
    .execute(&state.db);
  match row.await {
    Ok(_)  => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn save(state : &Data<AppState>,permissions : Permissions) -> Result<(),Box<dyn Error>> {
  let Permissions{
      id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem
  } = permissions;
  let row = query!("
    INSERT INTO permissions(id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12);",
      id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem).execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn update(state : &Data<AppState>,permissions : Permissions) -> Result<(),Box<dyn Error>> {
  let Permissions{
      id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem
  } = permissions;
  let row = query!("
    UPDATE permissions SET
      define_problem                                        = $2,
      modify_department_problems                            = $3,
      read_department_problems                              = $4,
      access_history_all_departments_department_problems    = $5,
      access_history_all_departments_problems               = $6,
      access_history_department_department_problems         = $7,
      access_history_department_problems                    = $8,
      access_history_employees                              = $9,
      access_history_machines                               = $10,
      access_history_spare_parts                            = $11,
      write_department_problem                              = $12
    WHERE id        = $1;",
      id,
      define_problem,
      modify_department_problems,
      read_department_problems,
      access_history_all_departments_department_problems,
      access_history_all_departments_problems,
      access_history_department_department_problems,
      access_history_department_problems,
      access_history_employees,
      access_history_machines,
      access_history_spare_parts,
      write_department_problem).execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
