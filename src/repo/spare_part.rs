use std::error::Error;

use actix_web::web::Data;
use sqlx::query;
use uuid::Uuid;

use crate::{AppState, model::spare_part::SparePart};

pub async fn find_all_spare_parts(state : Data<AppState>) -> Vec<SparePart> {
    let query = "
        select
            id,
            name
        from spare_part
                ";
    match sqlx::query_as::<_, SparePart>(query).fetch_all(&state.db).await {
        Ok(problems) => problems,
        Err(_) => Vec::new()
    }
}


pub async fn save_spare_part_to_shift_problem(state : Data<AppState>,shift_problem_id : &Uuid,spare_part_id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    INSERT INTO shift_problem_spare_part(
        shift_problem_id,
        spare_part_id)
    VALUES($1,$2);",
    shift_problem_id,spare_part_id)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
