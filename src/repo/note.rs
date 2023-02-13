
use actix_web::web::Data;
use sqlx::query_as;
use uuid::Uuid;

use crate::AppState;
use rec::model::note::Note;


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
