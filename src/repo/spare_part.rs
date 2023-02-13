use std::error::Error;

use actix_web::web::Data;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::{model::spare_part::SparePart,
  crud_sync::{
    Cud,
    CudVersion,
    Table
  }
};

use super::syncing::record_version;

pub async fn find_all_spare_parts(state : &Data<AppState>) -> Option<Vec<SparePart>> {
    let query = "
        select
            id,
            name
        from spare_part
                ";
    match sqlx::query_as::<_, SparePart>(query).fetch_all(&state.db).await {
        Ok(problems) => Some(problems),
        Err(_) => None
    }
}

pub async fn fetch_spare_part_by_id(state : &Data<AppState>,id : Uuid) -> Option<SparePart> {
  let row = query_as!(SparePart,r#"
        select id,name
        from spare_part WHERE id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(part) =>Some(part),
    Err(_) => None
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

pub async fn save(state : &Data<AppState>,part : SparePart) -> Result<(),Box<dyn Error>> {
  let SparePart{id,name} = part;
  let row = query!("
    INSERT INTO spare_part(id,name)
    VALUES($1,$2);",
    id,name).execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Create,
        target_table : Table::SparePart,
        version_number : 0,
        target_id : id,
        other_target_id: None
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }
    },
    Err(err) => Err(err.into())
  }
}

pub async fn update(state : &Data<AppState>,part : SparePart) -> Result<(),Box<dyn Error>> {
  let SparePart{id,name} = part;
  let row = query!("
    UPDATE spare_part
    SET name = $2
    WHERE id = $1;",
    id,name).execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Update,
        target_table : Table::SparePart,
        version_number : 0,
        target_id : id,
        other_target_id: None
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }
    },
    Err(err) => Err(err.into())
  }
}

pub async fn delete(state : &Data<AppState>,id : Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    DELETE FROM spare_part
    WHERE id = $1;",
    id).execute(&state.db);
  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
        cud : Cud::Delete,
        target_table : Table::SparePart,
        version_number : 0,
        target_id : id,
        other_target_id: None
      }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
      }
    },
    Err(err) => Err(err.into())
  }
}
