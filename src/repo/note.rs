use std::error::Error;

use actix_web::web::Data;
use sqlx::query;
use uuid::Uuid;

use crate::AppState;

pub async fn save_note_to_shift_problem(state : Data<AppState>,shift_problem_id : &Uuid,note : String) -> Result<(),Box<dyn Error>> {
  let row = query!("
    INSERT INTO note(
        id,
        shift_problem_id,
        content)
    VALUES($1,$2,$3);",Uuid::new_v4(),
    shift_problem_id,note)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
