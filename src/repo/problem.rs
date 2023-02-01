use std::error::Error;

use actix_web::web::Data;
use sqlx::query;
use uuid::Uuid;

use crate::{AppState, model::problem::Probelm};

pub async fn find_all_probelms(state : Data<AppState>) -> Vec<Probelm> {
    let query = "
        select
            id,
            title,
            description
        from problem
                ";
    match sqlx::query_as::<_, Probelm>(query).fetch_all(&state.db).await {
        Ok(problems) => problems,
        Err(_) => Vec::new()
    }
}


pub async fn save_problem_to_shift_problem(state : Data<AppState>,shift_problem_id : &Uuid,problem_id : &Uuid) -> Result<(),Box<dyn Error>> {
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
