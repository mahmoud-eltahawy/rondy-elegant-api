use std::error::Error;

use actix_web::web::Data;
use rec::{crud_sync::{CudVersion,Cud,Table}, model::{note::DbNote, relations::{ShiftProblemProblem, ShiftProblemSparePart}}};
use sqlx::query;

use crate::{AppState, repo::syncing::record_version};

pub async fn save_spare_part_to_shift_problem(state : &Data<AppState>,
            ss : ShiftProblemSparePart) -> Result<(),Box<dyn Error>> {
  let ShiftProblemSparePart{shift_problem_id,spare_part_id} = ss;
  let row = query!("
    INSERT INTO shift_problem_spare_part(
        shift_problem_id,
        spare_part_id)
    VALUES($1,$2);",
    shift_problem_id,spare_part_id)
    .execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblemSparePart,
        version_number : 0,
        target_id : shift_problem_id,
        other_target_id: Some(spare_part_id)
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }

    },
    Err(err) => Err(err.into())
  }
}

pub async fn remove_spare_part_from_shift_problem(state : &Data<AppState>,
            ss : ShiftProblemSparePart) -> Result<(),Box<dyn Error>> {
  let ShiftProblemSparePart{shift_problem_id,spare_part_id} = ss;
  let row = query!("
    DELETE FROM shift_problem_spare_part
    WHERE shift_problem_id = $1 AND spare_part_id = $2;",
    shift_problem_id,spare_part_id)
    .execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblemSparePart,
        version_number : 0,
        target_id : shift_problem_id,
        other_target_id: Some(spare_part_id)
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }

    },
    Err(err) => Err(err.into())
  }
}

pub async fn save_problem_to_shift_problem(state : &Data<AppState>,
                sp : ShiftProblemProblem) -> Result<(),Box<dyn Error>> {
  let ShiftProblemProblem{problem_id,shift_problem_id} = sp;
  let row = query!("
    INSERT INTO shift_problem_problem(
        shift_problem_id,
        problem_id)
    VALUES($1,$2);",
    shift_problem_id,problem_id)
    .execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblemProblem,
        version_number : 0,
        target_id : shift_problem_id,
        other_target_id: Some(problem_id)
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }
    },
    Err(err) => Err(err.into())
  }
}

pub async fn remove_problem_from_shift_problem(state : &Data<AppState>,
            sp : ShiftProblemProblem) -> Result<(),Box<dyn Error>> {
  let ShiftProblemProblem{problem_id,shift_problem_id} = sp;
  let row = query!("
    DELETE FROM shift_problem_problem
    WHERE shift_problem_id = $1 AND problem_id = $2;",
    shift_problem_id,problem_id)
    .execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblemProblem,
        version_number : 0,
        target_id : shift_problem_id,
        other_target_id: Some(problem_id)
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }

    },
    Err(err) => Err(err.into())
  }
}

pub async fn save_note_to_shift_problem(state : &Data<AppState>,
                              note : DbNote) -> Result<(),Box<dyn Error>> {
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
    VALUES($1,$2,$3);",id,
    shift_problem_id,content)
    .execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Create,
        target_table : Table::ShiftProblemNote,
        version_number : 0,
        target_id : shift_problem_id,
        other_target_id: Some(id)
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }
    },
    Err(err) => Err(err.into())
  }
}

pub async fn remove_note_to_shift_problem(state : &Data<AppState>,
                              note : DbNote) -> Result<(),Box<dyn Error>> {
  let DbNote{shift_id:_,id,shift_problem_id,content: _} = note;
  let shift_problem_id = match shift_problem_id {
    Some(id) => id,
    None     => return Err("not qualified params".to_owned().into())
  };
  let row = query!("
    DELETE FROM note
    WHERE id = $1 AND shift_problem_id = $2;",
    id,
    shift_problem_id)
    .execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::ShiftProblemNote,
        version_number : 0,
        target_id : shift_problem_id,
        other_target_id: Some(id)
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }
    },
    Err(err) => Err(err.into())
  }
}
