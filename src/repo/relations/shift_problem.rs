use std::error::Error;

use actix_web::web::Data;
use sqlx::query;
use uuid::Uuid;

use crate::AppState;

pub async fn save_problem_to_shift_problem(state : &Data<AppState>,
                problem_id: &Uuid,shift_problem_id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    INSERT INTO shift_problem_problem(
        shift_problem_id,
        problem_id)
    VALUES($1,$2);",
    shift_problem_id,problem_id)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn remove_problem_from_shift_problem(state : &Data<AppState>,
            problem_id: &Uuid,shift_problem_id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    DELETE FROM shift_problem_problem
    WHERE shift_problem_id = $1 AND problem_id = $2;",
    shift_problem_id,problem_id)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn save_spare_part_to_shift_problem(state : &Data<AppState>,
            part_id: &Uuid,shift_problem_id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    INSERT INTO shift_problem_spare_part(
        shift_problem_id,
        spare_part_id)
    VALUES($1,$2);",
    shift_problem_id,part_id)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn remove_spare_part_from_shift_problem(state : &Data<AppState>,
            part_id: &Uuid,shift_problem_id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    DELETE FROM shift_problem_spare_part
    WHERE shift_problem_id = $1 AND spare_part_id = $2;",
    shift_problem_id,part_id)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
