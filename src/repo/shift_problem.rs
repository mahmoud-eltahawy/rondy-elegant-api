use std::error::Error;

use actix_web::web::Data;
use sqlx::{query_as, query};
use uuid::Uuid;

use crate::AppState;
use rec::{
  model::{shift_problem::{
    DbShiftProblem,
    MinimamlShiftProblem
  }, note::DbNote, relations::{ShiftProblemProblem, ShiftProblemSparePart}},
  crud_sync::{
    CudVersion,
    Cud,
    Table
  }
};

use super::{
    note::fetch_note_by_shift_problem_id,
    employee::get_employee_department_id_by_id,
    syncing::record_version,
    relations::shift_problem::{
    save_spare_part_to_shift_problem,
    save_problem_to_shift_problem,
    save_note_to_shift_problem
  }
};

async fn fetch_department_db_shift_problems_by_writer_and_shift_ids(state : &Data<AppState>,
                  writer_id : Uuid,shift_id : Uuid) -> Result<Vec<DbShiftProblem>,Box<dyn Error>> {
    let department_id = match get_employee_department_id_by_id(state, writer_id).await {
      Ok(id) => id,
      Err(err) => return Err(err.into())
    };
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
        WHERE shift_id = $1 AND
        writer_id IN (SELECT id FROM employee WHERE department_id = $2)
        "#,shift_id,department_id).fetch_all(&state.db).await {
     Ok(problems) => Ok(problems),
     Err(err) => Err(err.into())
   }
}

async fn fetch_db_shift_problem_by_id(state : &Data<AppState>,
                  id : Uuid) -> Result<DbShiftProblem,Box<dyn Error>> {
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
        WHERE id = $1;
        "#,id).fetch_one(&state.db).await {
     Ok(problem) => Ok(problem),
     Err(err) => Err(err.into())
   }
}


pub async fn save_db_shift_problem(state : &Data<AppState>,
                    shift_problem : &DbShiftProblem) -> Result<(),Box<dyn Error>> {
  let DbShiftProblem{
    id,
    shift_id,
    writer_id,
    maintainer_id,
    machine_id,
    begin_time,
    end_time
  } = shift_problem;
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
            shift_problem_id,
            shift_id,
            writer_id,
            maintainer_id,
            machine_id,
            begin_time,
            end_time
        ).execute(&state.db);
        match row.await {
            Ok(_) =>{
              match record_version(state, CudVersion{
                cud : Cud::Create,
                target_table : Table::ShiftProblem,
                version_number : 0,
                target_id : id.unwrap().to_owned(),
                other_target_id: None
              }).await {
                Ok(_) => Ok(()),
                Err(err) => Err(err.into())
              }
            },
            Err(err) => Err(err.into())
        }
    },
    None => Err("shift problem id is null".into())
  }
}
pub async fn update_db_shift_problem(state : &Data<AppState>,
                    shift_problem : &DbShiftProblem) -> Result<(),Box<dyn Error>> {
  let DbShiftProblem{
    id,
    shift_id,
    writer_id,
    maintainer_id,
    machine_id,
    begin_time,
    end_time
  } = shift_problem;
  match id {
    Some(shift_problem_id) => {
        let row = query!("
            UPDATE shift_problem SET
            shift_id = $2,
            writer_id = $3,
            maintainer_id = $4,
            machine_id = $5,
            begin_time = $6,
            end_time = $7
            WHERE id = $1;",
            shift_problem_id,
            shift_id,
            writer_id,
            maintainer_id,
            machine_id,
            begin_time,
            end_time
        ).execute(&state.db);
        match row.await {
            Ok(_) =>{
              match record_version(state, CudVersion{
                cud : Cud::Update,
                target_table : Table::ShiftProblem,
                version_number : 0,
                target_id : id.unwrap().to_owned(),
                other_target_id: None
              }).await {
                Ok(_) => Ok(()),
                Err(err) => Err(err.into())
              }
            },
            Err(err) => Err(err.into())
        }
    },
    None => Err("shift problem id is null".into())
  }
}
pub async fn delete_db_shift_problem(state : &Data<AppState>,
                    shift_problem_id : &Uuid) -> Result<(),Box<dyn Error>> {
  match query!("
      DELETE FROM shift_problem
      WHERE id = $1;",
      shift_problem_id
  ).execute(&state.db).await {
      Ok(_) =>{
        match record_version(state, CudVersion{
          cud : Cud::Delete,
          target_table : Table::ShiftProblem,
          version_number : 0,
          target_id : shift_problem_id.to_owned(),
          other_target_id: None
        }).await {
          Ok(_) => Ok(()),
          Err(err) => Err(err.into())
        }
      },
      Err(err) => Err(err.into())
  }
}

pub async fn save_shift_problem(state : &Data<AppState>,shift_problem : MinimamlShiftProblem) -> Result<Uuid,Box<dyn Error>> {
  let shift_problem_id = Uuid::new_v4();
  let MinimamlShiftProblem{id : _,shift_id,writer_id,maintainer_id,machine_id,begin_time,end_time,
                           problems_ids,spare_parts_ids,note} = shift_problem;
  let db_shift_problem = DbShiftProblem{id : Some(shift_problem_id),shift_id,writer_id,
                           maintainer_id,machine_id,begin_time,end_time};
  match save_db_shift_problem(&state, &db_shift_problem).await {
    Ok(_) => {
        for problem_id in problems_ids{
          let _ = save_problem_to_shift_problem(&state,
                     ShiftProblemProblem{problem_id,shift_problem_id}).await;
        }
        if let Some(spare_parts_ids) = spare_parts_ids {
          for spare_part_id in spare_parts_ids {
            let _ = save_spare_part_to_shift_problem(&state,
                      ShiftProblemSparePart{shift_problem_id,spare_part_id}).await;
          }
        }
        if let Some(note) = note {
          let _ = save_note_to_shift_problem(&state,
                        DbNote{id : note.id,
                               content: note.content,
                               shift_problem_id: Some(shift_problem_id),
                               shift_id: None}).await;
        }
        Ok(shift_problem_id)
    },
    Err(err) => Err(err)
  }
}

pub async fn fetch_department_shift_problems_by_writer_and_shift_id(state : &Data<AppState>,
                                    writer_id : Uuid,shift_id : Uuid) -> Result<Vec<MinimamlShiftProblem>,Box<dyn Error>> {
  let db_problems = fetch_department_db_shift_problems_by_writer_and_shift_ids(state, writer_id, shift_id);
  let db_problems = match db_problems.await {
    Ok(problems) => problems,
    Err(err) => return Err(err.into())
  };

  let mut shift_problems = Vec::<MinimamlShiftProblem>::new();
  for db_p in db_problems {
    match db_to_minimal_shift_problem(state, db_p).await{
      Ok(problem) => shift_problems.push(problem),
      Err(err)    => return Err(err)
    }
  }
  Ok(shift_problems)
}

pub async fn fetch_shift_problem_by_id(state : &Data<AppState>,
                                    id : Uuid) -> Result<MinimamlShiftProblem,Box<dyn Error>> {
  let problem = fetch_db_shift_problem_by_id(state, id);
  let problem = match problem.await {
    Ok(problem) => problem,
    Err(err) => return Err(err.into())
  };
  match db_to_minimal_shift_problem(state, problem).await{
    Ok(problem) => Ok(problem),
    Err(err)    => return Err(err)
  }
}

async fn db_to_minimal_shift_problem(state : &Data<AppState>,
                                problem : DbShiftProblem) -> Result<MinimamlShiftProblem, Box<dyn Error>>{
    let DbShiftProblem{id,shift_id,writer_id,machine_id,begin_time,end_time,maintainer_id} = problem;
    let shift_problem_id = id.unwrap();
    let problems_ids = fetch_problems_ids_by_shift_problem_id(&state, &shift_problem_id);
    let problems_ids = match problems_ids.await {
      Ok(problems) => problems,
      Err(err) => return Err(err.into())
    };
    let spare_parts_ids = fetch_spare_parts_ids_by_shift_problem_id(&state, &shift_problem_id);
    let spare_parts_ids = match spare_parts_ids.await {
      Ok(problems) => Some(problems),
      Err(err) => return Err(err)
    };

    let note = fetch_note_by_shift_problem_id(&state, &shift_problem_id).await;

    Ok(MinimamlShiftProblem{
      id,
      shift_id,
      writer_id,
      machine_id,
      begin_time,
      end_time,
      maintainer_id,
      problems_ids,
      spare_parts_ids,
      note
    })
}

async fn fetch_problems_ids_by_shift_problem_id(state : &Data<AppState>,
                        shift_problem_id : &Uuid) -> Result<Vec<Uuid>,Box<dyn Error>> {
  let row = query!("
    SELECT problem_id FROM shift_problem_problem WHERE shift_problem_id = $1",
    shift_problem_id).fetch_all(&state.db);
  match row.await {
    Ok(problems_ids_records) => Ok(problems_ids_records.into_iter().map(|p| {p.problem_id}).collect()),
    Err(err) => Err(err.into())
  }
}

pub async fn fetch_spare_parts_ids_by_shift_problem_id(state : &Data<AppState>,
                            shift_problem_id : &Uuid) -> Result<Vec<Uuid>,Box<dyn Error>> {
  let row = query!("
    SELECT spare_part_id FROM shift_problem_spare_part WHERE shift_problem_id = $1",
    shift_problem_id).fetch_all(&state.db);
  match row.await {
    Ok(spare_parts_ids_records) => Ok(spare_parts_ids_records
                            .into_iter().map(|sp| {sp.spare_part_id}).collect()),
    Err(err) => Err(err.into())
  }
}
