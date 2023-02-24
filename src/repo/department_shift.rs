use actix_web::web::Data;
use sqlx::{query_as, query,Error};
use uuid::Uuid;

use crate::AppState;
use rec::{
  model::shift::{DepartmentShift, DateOrder, DbShift, Shift},
  crud_sync::{
    CudVersion,
    Cud,Table
  }, timer::{get_relative_now, get_current_date, get_current_order}
};

use super::syncing::record_version;

pub async fn find_shift_by_id(state : &Data<AppState>,id : Uuid) -> Result<Option<Shift>,Error> {
    let shift = query_as!(DbShift,r#"
        select
            id,
            shift_order,
            shift_date
        from shift where id = $1
    "#,id).fetch_one(&state.db).await?;
    Ok(Shift::new(shift))
}

pub async fn find_department_shift_by_id(state : &Data<AppState>,id : Uuid) -> Result<DepartmentShift,Error> {
    let result = query_as!(DepartmentShift,r#"
        select * from department_shift where id = $1
    "#,id).fetch_one(&state.db).await?;
    Ok(result)
}

pub async fn save_department_shift(state : &Data<AppState>,department_id : Uuid) -> Result<(),Error> {
    let id = Uuid::new_v4();
    let shift_id = save_or_get_shift_id(state).await?;
    query!("
        INSERT INTO department_shift(id,shift_id,department_id)
        VALUES($1,$2,$3)
    ",id,shift_id,department_id).execute(&state.db).await?;
    record_version(state, CudVersion{
      cud               : Cud::Create,
      target_table      : Table::DepartmentShift,
      version_number    : 0,
      target_id         : id,
      other_target_id   : None
    }).await?;
    Ok(())
}

pub async fn remove_department_shift(state : &Data<AppState>,id : Uuid) -> Result<(),Error> {
    query!("DELETE FROM department WHERE id = $1",id)
        .execute(&state.db).await?;
    record_version(state, CudVersion{
      cud             : Cud::Delete,
      target_table    : Table::DepartmentShift,
      version_number  : 0,
      target_id       : id,
      other_target_id : None
    }).await?;
    Ok(())
}

async fn save_or_get_shift_id(state : &Data<AppState>) -> Result<Uuid,Error>{
  let now   = get_relative_now();
  let date  = get_current_date(now);
  let order = get_current_order(now);
  if let Some(date) = date {
    let order = order as i16;
    match find_shift_id_by_date_and_order(state, DateOrder{date,order}).await {
      Ok(id) => Ok(id),
      Err(_) => {
        let id = Uuid::new_v4();
        save_shift(state, DbShift{
          id,
          shift_date: date,
          shift_order: order
        }).await?;
        Ok(id)
      }
    }
  } else {
    Err(Error::PoolClosed)
  }
}

async fn find_shift_id_by_date_and_order(state : &Data<AppState>,
                                        od : DateOrder) -> Result<Uuid,Error> {
  let DateOrder{date,order} = od;
  let shift = query_as!(DbShift,r#"
      select
          id,
          shift_order,
          shift_date
      from shift where shift_date = $1 and shift_order = $2
  "#,date , order).fetch_one(&state.db).await?;
  Ok(shift.id)
}

async fn save_shift(state : &Data<AppState>,shift : DbShift) -> Result<(),Error> {
  let DbShift{id,shift_date,shift_order} = shift;
  query!("
      INSERT INTO shift(id,shift_order,shift_date)
      VALUES($1,$2,$3)
  ",id,shift_order,shift_date).execute(&state.db).await?;
  record_version(state, CudVersion{
    cud : Cud::Create,
    target_table : Table::Shift,
    version_number : 0,
    target_id : id,
    other_target_id: None
  }).await?;
  Ok(())
}
