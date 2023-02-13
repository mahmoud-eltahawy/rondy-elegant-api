use actix_web::web::Data;
use sqlx::{query_as, query,Error};
use uuid::Uuid;

use crate::AppState;
use rec::{model::machine::Machine,
          crud_sync::{CudVersion, Cud,Table}};

use super::syncing::record_version;

pub async fn save(state : &Data<AppState>,machine : Machine) -> Result<(),Error> {
  let Machine{id,name} = machine;
  let row = query!("
    INSERT INTO machine(
    id,
    name)
    VALUES($1,$2);",
                  id,
                  name
  ).execute(&state.db);

  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
      cud : Cud::Create,
      target_table : Table::Machine,
      target_id : id,
      other_target_id : None,
      version_number : 0
  }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
      }
    },
    Err(err) => Err(err)
  }
}

pub async fn update(state : &Data<AppState>,machine : Machine) -> Result<(),Error> {
  let Machine{id,name} = machine;
  let row = query!("
    UPDATE machine SET
    name = $2
    WHERE id = $1;",
    id,
    name
  ).execute(&state.db);

  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
      cud : Cud::Update,
      target_table : Table::Machine,
      target_id : id,
      other_target_id : None,
      version_number : 0
  }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
      }
    },
    Err(err) => Err(err)
  }
}

pub async fn delete(state : &Data<AppState>,id : Uuid) -> Result<(),Error> {
  let row = query!("
    DELETE FROM machine
    WHERE id = $1;",
    id
  ).execute(&state.db);

  match row.await {
    Ok(_) =>{
      match record_version(state, CudVersion{
      cud : Cud::Delete,
      target_table : Table::Machine,
      target_id : id,
      other_target_id : None,
      version_number : 0
  }).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
      }
    },
    Err(err) => Err(err)
  }
}

pub async fn fetch_machine_by_id(state : &Data<AppState>,id : Uuid) -> Option<Machine> {
  let row = query_as!(Machine,r#"
        select id,name
        from machine WHERE id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(machine) =>Some(machine),
    Err(_) => None
  }
}

pub async fn fetch_all(state : &Data<AppState>) -> Option<Vec<Machine>> {
  let row = query_as!(Machine,r#"
        select id,name
        from machine"#)
    .fetch_all(&state.db);
  match row.await {
    Ok(machine) =>Some(machine),
    Err(_) => None
  }
}
