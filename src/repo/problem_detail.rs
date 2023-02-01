use std::{error::Error, result};

use actix_web::web::Data;
use sqlx::{query_as, query};
use uuid::Uuid;

use crate::{AppState, model::problem_detail::{DbShiftProblem, MinimamlShiftProblem}};

use super::{
    problem::save_problem_to_shift_problem,
    spare_part::save_spare_part_to_shift_problem,
    note::save_note_to_shift_problem};

pub async fn find_all_db_shift_problem(state : Data<AppState>) -> Result<Vec<DbShiftProblem>,Box<dyn Error>> {
    match query_as!(DbShiftProblem,r#"
        select
            id as "id?",
            shift_id,
            writer_id,
            maintainer_id,
            machine_id,
            begin_time,
            end_time
        from shift_problem
        "#).fetch_all(&state.db).await {
     Ok(employees) => Ok(employees),
     Err(err) => Err(err.into())
   }
}


pub async fn save_db_shift_problem(state : Data<AppState>,shift_problem : &DbShiftProblem) -> Result<(),Box<dyn Error>> {
  let DbShiftProblem{id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time} = shift_problem;
  match id {
    Some(shift_problem_id) => {
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
            shift_problem_id,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time)
            .execute(&state.db);
        match row.await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into())
        }
    },
    None => Err("shift problem id is null".into())
  }
}

pub async fn save_shift_problem(state : Data<AppState>,shift_problem : MinimamlShiftProblem) -> Result<Uuid,Box<dyn Error>> {
  let shift_problem_id = Uuid::new_v4();
  let MinimamlShiftProblem{id : _,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time,
                           problems_ids,spare_parts_ids,note} = shift_problem;
  let db_shift_problem = DbShiftProblem{id : Some(shift_problem_id),shift_id,writer_id,
                           maintainer_id,machine_id,begin_time,end_time};
  match save_db_shift_problem(state.clone(), &db_shift_problem).await {
    Ok(_) => {
        for problem_id in problems_ids{
          let _ = save_problem_to_shift_problem(state.clone(), &shift_problem_id, &problem_id).await;
        }
        if let Some(spare_parts_ids) = spare_parts_ids {
          for spare_part_id in spare_parts_ids {
            let _ = save_spare_part_to_shift_problem(state.clone(),&shift_problem_id, &spare_part_id).await;
          }
        }
        if let Some(note) = note {
          let _ = save_note_to_shift_problem(state, &shift_problem_id, note).await;
        }
        Ok(shift_problem_id)
    },
    Err(err) => Err(err)
  }
}
