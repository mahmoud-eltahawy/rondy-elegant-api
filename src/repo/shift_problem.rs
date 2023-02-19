use std::error::Error;

use actix_web::web::Data;
use rec::model::shift_problem::ShiftProblem;
use sqlx::{query_as, query};
use uuid::Uuid;

use crate::AppState;

pub async fn find_shift_problem_by_id(state : &Data<AppState>,
                  id : Uuid) -> Result<ShiftProblem,Box<dyn Error>> {
    match query_as!(ShiftProblem,r#"
        select
            id,
            shift_id,
            writer_id,
            maintainer_id,
            machine_id,
            begin_time,
            end_time
        from shift_problem
        WHERE id = $1;
        "#,id).fetch_one(&state.db).await {
     Ok(problem) => Ok(problem),
     Err(err) => Err(err.into())
   }
}


pub async fn save_shift_problem(state : &Data<AppState>,
                    shift_problem : &ShiftProblem) -> Result<(),Box<dyn Error>> {
  let ShiftProblem{
    id,
    shift_id,
    writer_id,
    maintainer_id,
    machine_id,
    begin_time,
    end_time
  } = shift_problem;
  let row = query!("
      INSERT INTO shift_problem(
          id,
          shift_id,
          writer_id,
          maintainer_id,
          machine_id,
          begin_time,
          end_time)
      VALUES($1,$2,$3,$4,$5,$6,$7);",
      id,
      shift_id,
      writer_id,
      maintainer_id,
      machine_id,
      begin_time,
      end_time
  ).execute(&state.db);
  match row.await {
      Ok(_) => Ok(()),
      Err(err) => Err(err.into())
  }
}
pub async fn update_shift_problem(state : &Data<AppState>,
                    shift_problem : &ShiftProblem) -> Result<(),Box<dyn Error>> {
  let ShiftProblem{
    id,
    shift_id,
    writer_id,
    maintainer_id,
    machine_id,
    begin_time,
    end_time
  } = shift_problem;
  let row = query!("
      UPDATE shift_problem SET
      shift_id = $2,
      writer_id = $3,
      maintainer_id = $4,
      machine_id = $5,
      begin_time = $6,
      end_time = $7
      WHERE id = $1;",
      id,
      shift_id,
      writer_id,
      maintainer_id,
      machine_id,
      begin_time,
      end_time
  ).execute(&state.db);
  match row.await {
      Ok(_) => Ok(()),
      Err(err) => Err(err.into())
  }
}
pub async fn delete_shift_problem(state : &Data<AppState>,
                    shift_problem_id : &Uuid) -> Result<(),Box<dyn Error>> {
  match query!("
      DELETE FROM shift_problem
      WHERE id = $1;",
      shift_problem_id
  ).execute(&state.db).await {
      Ok(_) => Ok(()),
      Err(err) => Err(err.into())
  }
}
