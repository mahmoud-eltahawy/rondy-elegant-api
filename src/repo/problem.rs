use std::error::Error;

use actix_web::web::Data;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{AppState, model::problem::Probelm};

pub async fn find_all_probelms(state : Data<AppState>) -> Vec<Probelm> {
    let query = "
        select
            id,
            title,
            description
        from problem
                ";
    match sqlx::query_as::<_, Probelm>(query).fetch_all(&state.db).await {
        Ok(problems) => problems,
        Err(_) => Vec::new()
    }
}

pub async fn fetch_problem_by_id(state : Data<AppState>,id : Uuid) -> Option<Probelm> {
  let row = query_as!(Probelm,r#"
        select id,title ,description
        from problem WHERE id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(machine) =>Some(machine),
    Err(_) => None
  }
}


pub async fn save_problem_to_shift_problem(state : Data<AppState>,shift_problem_id : &Uuid,problem_id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    INSERT INTO shift_problem_problem(
        shift_problem_id,
        problem_id)
    VALUES($1,$2);",
    shift_problem_id,problem_id)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn save(state : Data<AppState>,problem : Probelm) -> Result<(),Box<dyn Error>> {
  let Probelm{id,title,description} = problem;
  let row = query!("
    INSERT INTO problem(id,title,description) VALUES($1,$2,$3);",
    id,title,description)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

struct ProblemId{
  problem_id : Uuid
}

pub async fn fetch_problems_ids_by_shift_problem_id(state : Data<AppState>,shift_problem_id : &Uuid) -> Result<Vec<Uuid>,Box<dyn Error>> {
  let row = query_as!(ProblemId,"
    SELECT problem_id FROM shift_problem_problem WHERE shift_problem_id = $1",
    shift_problem_id).fetch_all(&state.db);
  match row.await {
    Ok(problems_ids) => Ok(problems_ids.into_iter().map(|p| {p.problem_id}).collect()),
    Err(err) => Err(err.into())
  }
}
