use std::error::Error;

use actix_web::web::Data;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{AppState, model::spare_part::SparePart};

pub async fn find_all_spare_parts(state : Data<AppState>) -> Vec<SparePart> {
    let query = "
        select
            id,
            name
        from spare_part
                ";
    match sqlx::query_as::<_, SparePart>(query).fetch_all(&state.db).await {
        Ok(problems) => problems,
        Err(_) => Vec::new()
    }
}

pub async fn fetch_spare_part_by_id(state : Data<AppState>,id : Uuid) -> Option<SparePart> {
  let row = query_as!(SparePart,r#"
        select id,name
        from spare_part WHERE id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(part) =>Some(part),
    Err(_) => None
  }
}


pub async fn save_spare_part_to_shift_problem(state : Data<AppState>,shift_problem_id : &Uuid,spare_part_id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    INSERT INTO shift_problem_spare_part(
        shift_problem_id,
        spare_part_id)
    VALUES($1,$2);",
    shift_problem_id,spare_part_id)
    .execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

struct SparePartId{
  spare_part_id : Uuid
}

pub async fn fetch_spare_parts_ids_by_shift_problem_id(state : Data<AppState>,
                            shift_problem_id : &Uuid) -> Result<Vec<Uuid>,Box<dyn Error>> {
  let row = query_as!(SparePartId,"
    SELECT spare_part_id FROM shift_problem_spare_part WHERE shift_problem_id = $1",
    shift_problem_id).fetch_all(&state.db);
  match row.await {
    Ok(spare_parts_ids) => Ok(spare_parts_ids.into_iter().map(|sp| {sp.spare_part_id}).collect()),
    Err(err) => Err(err.into())
  }
}
