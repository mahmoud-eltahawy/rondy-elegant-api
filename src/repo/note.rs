use std::error::Error;

use actix_web::web::Data;
use sqlx::{query_as, query};
use uuid::Uuid;

use crate::AppState;
use rec::model::note::{Note, DbNote};

pub async fn fetch_note_by_id(state : &Data<AppState>,
                        id : &Uuid) -> Option<DbNote<Uuid>> {
  let row = query_as!(DbNote,r#"
    SELECT * FROM note WHERE id = $1;"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(name) => Some(name),
    Err(_) => None
  }
}

pub async fn save_note_to_shift_problem(state : &Data<AppState>,
                              note : &DbNote<Uuid>) -> Result<(),Box<dyn Error>> {
  let DbNote{shift_id:_,id,shift_problem_id,content} = note;
  let shift_problem_id = match shift_problem_id {
    Some(id) => id,
    None     => return Err("not qualified params".to_owned().into())
  };
  let row = query!("
    INSERT INTO note(
        id,
        shift_problem_id,
        content)
    VALUES($1,$2,$3);",
    id,shift_problem_id,content)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn save_note_to_shift(state : &Data<AppState>,
                              note : &DbNote<Uuid>) -> Result<(),Box<dyn Error>> {
  let DbNote{id,shift_id,shift_problem_id : _,content} = note;
  let shift_id = match shift_id {
    Some(id) => id,
    None     => return Err("not qualified params".to_owned().into())
  };
  let row = query!("
    INSERT INTO note(
        id,
        shift_id,
        content)
    VALUES($1,$2,$3);",
    id,shift_id,content)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn update_note(state : &Data<AppState>,
                              note : &Note<Uuid>) -> Result<(),Box<dyn Error>> {
  let Note{id,content} = note;
  let row = query!("
    UPDATE note SET content = $2 WHERE id =$1;"
    ,id,content)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn remove_note(state : &Data<AppState>,
                              id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    DELETE FROM note WHERE id = $1",
    id)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
