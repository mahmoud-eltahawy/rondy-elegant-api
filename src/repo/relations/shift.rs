use std::error::Error;

use actix_web::web::Data;
use rec::{
    model::note::DbNote,
    crud_sync::{CudVersion,Cud,Table}};
use sqlx::query;

use crate::{AppState, repo::syncing::record_version};



pub async fn save_note_to_shift(state : &Data<AppState>,
                              note : DbNote) -> Result<(),Box<dyn Error>> {
  let DbNote{shift_id,id,shift_problem_id: _,content} = note;
  let shift_id = match shift_id {
    Some(id) => id,
    None     => return Err("not qualified params".to_owned().into())
  };
  let row = query!("
    INSERT INTO note(
        id,
        shift_id,
        content)
    VALUES($1,$2,$3);",id,
    shift_id,content)
    .execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftNote,
        version_number : 0,
        target_id : shift_id,
        other_target_id: Some(id)
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }
    },
    Err(err) => Err(err.into())
  }
}

pub async fn remove_note_to_shift(state : &Data<AppState>,
                              note : DbNote) -> Result<(),Box<dyn Error>> {
  let DbNote{shift_id,id,shift_problem_id:_,content: _} = note;
  let shift_id = match shift_id {
    Some(id) => id,
    None     => return Err("not qualified params".to_owned().into())
  };
  let row = query!("
    DELETE FROM note
    WHERE id = $1 AND shift_id = $2;",
    id,
    shift_id)
    .execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblemNote,
        version_number : 0,
        target_id : shift_id,
        other_target_id: Some(id)
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }
    },
    Err(err) => Err(err.into())
  }
}
