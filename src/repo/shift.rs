use actix_web::web::Data;
use chrono::NaiveDate;
use sqlx::{query_as, query};
use uuid::Uuid;

use crate::{AppState, model::shift::DbShift, timer::ShiftOrder};

pub async fn find_all_db_shifts(state : Data<AppState>) -> Vec<DbShift> {
    match query_as!(DbShift,r#"
        select
            id,
            shift_order,
            shift_date
        from shift
    "#).fetch_all(&state.db).await {
        Ok(shift) => shift,
        Err(_) => Vec::new()
    }
}

pub async fn find_db_shift_by_id(state : Data<AppState>,id : Uuid) -> Option<DbShift> {
    match query_as!(DbShift,r#"
        select
            id,
            shift_order,
            shift_date
        from shift where id = $1
    "#,id).fetch_one(&state.db).await {
        Ok(shift) => Some(shift),
        Err(_) => None
    }
}

pub async fn find_db_shift_by_date_and_order(state : Data<AppState>,date :NaiveDate, order : ShiftOrder) -> Option<DbShift> {
    match query_as!(DbShift,r#"
        select
            id,
            shift_order,
            shift_date
        from shift where shift_date = $1 and shift_order = $2
    "#,date , order as i16).fetch_one(&state.db).await {
        Ok(shift) => Some(shift),
        Err(_) => None
    }
}

pub async fn save_db_shift(state : Data<AppState>,shift : DbShift) -> Option<DbShift> {
    let DbShift{id,shift_date,shift_order} = shift;
    match query!("
        INSERT INTO shift(id,shift_order,shift_date)
        VALUES($1,$2,$3)
    ",id,shift_order,shift_date).execute(&state.db).await {
        Ok(_) => Some(shift),
        Err(_) => None
    }
}
