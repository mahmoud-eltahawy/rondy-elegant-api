use std::error::Error;

use actix_web::web::Data;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::model::spare_part::SparePart;

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

pub async fn save(state : &Data<AppState>,part : &SparePart) -> Result<(),Box<dyn Error>> {
  let SparePart{id,name} = part;
  let row = query!("
    INSERT INTO spare_part(id,name)
    VALUES($1,$2);",
    id,name).execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn update(state : &Data<AppState>,part : &SparePart) -> Result<(),Box<dyn Error>> {
  let SparePart{id,name} = part;
  let row = query!("
    UPDATE spare_part
    SET name = $2
    WHERE id = $1;",
    id,name).execute(&state.db);
  match row.await {
    Ok(_) =>Ok(()),
    Err(err) => Err(err.into())
  }
}

pub async fn delete(state : &Data<AppState>,id : &Uuid) -> Result<(),Box<dyn Error>> {
  let row = query!("
    DELETE FROM spare_part
    WHERE id = $1;",
    id).execute(&state.db);
  match row.await {
    Ok(_) => Ok(()),
    Err(err) => Err(err.into())
  }
}
