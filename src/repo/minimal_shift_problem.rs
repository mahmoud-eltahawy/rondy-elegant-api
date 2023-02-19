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
