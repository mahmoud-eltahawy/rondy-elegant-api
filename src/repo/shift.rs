use actix_web::web::Data;
use sqlx::{query_as, query,Error};
use uuid::Uuid;

use crate::AppState;
use rec::{
  model::shift::{DbShift,Shift,DateOrder},
  timer::{
    get_relative_now,
    get_current_date,
    get_current_order
  }, crud_sync::{
    CudVersion,
    Cud,
    Table
  }
};

use super::syncing::record_version;

pub async fn get_or_save_db_shift(state : &Data<AppState>) -> Option<Shift>{
  let now = get_relative_now();
  let date = get_current_date(now);
  let order = get_current_order(now);
  if let Some(date) = date {
    let order = order as i16;
    match find_db_shift_by_date_and_order(state, DateOrder{date,order}).await {
      Some(shift) => Some(shift),
      None        => save_db_shift(state, DbShift{
          id: Uuid::new_v4(),
          shift_date: date,
          shift_order: order
        }).await
      }
    } else {
    None
  }
}

pub async fn find_db_shift_by_id(state : &Data<AppState>,id : Uuid) -> Option<Shift> {
    match query_as!(DbShift,r#"
        select
            id,
            shift_order,
            shift_date
        from shift where id = $1
    "#,id).fetch_one(&state.db).await {
        Ok(shift) => Shift::new(shift),
        Err(_) => None
    }
}

pub async fn find_db_shift_by_date_and_order(state : &Data<AppState>,
                                        od : DateOrder) -> Option<Shift> {
    let DateOrder{date,order} = od;
    match query_as!(DbShift,r#"
        select
            id,
            shift_order,
            shift_date
        from shift where shift_date = $1 and shift_order = $2
    "#,date , order).fetch_one(&state.db).await {
        Ok(shift) => Shift::new(shift),
        Err(_) => None
    }
}

async fn save_db_shift(state : &Data<AppState>,shift : DbShift) -> Option<Shift> {
    let DbShift{id,shift_date,shift_order} = shift;
    match query!("
        INSERT INTO shift(id,shift_order,shift_date)
        VALUES($1,$2,$3)
    ",id,shift_order,shift_date).execute(&state.db).await {
        Ok(_) =>{
          match record_version(state, CudVersion{
            cud : Cud::Create,
            target_table : Table::Shift,
            version_number : 0,
            target_id : id,
            other_target_id: None
          }).await {
            Ok(_) => Shift::new(shift),
            Err(_) => None
          }
        },
        Err(_) => None
    }
}

pub async fn remove_db_shift(state : &Data<AppState>,id : Uuid) -> Result<(),Error> {
    match query!("
        DELETE FROM shift WHERE id = $1
    ",id).execute(&state.db).await {
        Ok(_) =>{
          match record_version(state, CudVersion{
            cud : Cud::Delete,
            target_table : Table::Shift,
            version_number : 0,
            target_id : id,
            other_target_id: None
          }).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::PoolClosed)
          }
        },
        Err(_) => Err(Error::PoolClosed)
    }
}
