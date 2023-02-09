use std::error::Error;

use actix_web::web::Data;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::model::note::Note;

pub async fn save_note_to_shift_problem(state : &Data<AppState>,
                  shift_problem_id : &Uuid,note : Note) -> Result<(),Box<dyn Error>> {
  let Note{id,content} = note;
  let row = query!("
    INSERT INTO note(
        id,
        shift_problem_id,
        content)
    VALUES($1,$2,$3);",id,
    shift_problem_id,content)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn fetch_note_by_shift_problem_id(state : &Data<AppState>,
                        shift_problem_id : &Uuid) -> Option<Note> {
  let row = query_as!(Note,r#"
    SELECT id ,content FROM note WHERE shift_problem_id = $1;"#,shift_problem_id)
    .fetch_one(&state.db);
  match row.await {
    Ok(name) => Some(name),
    Err(_) => None
  }
}
