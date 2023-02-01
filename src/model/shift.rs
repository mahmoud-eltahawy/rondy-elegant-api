use chrono::NaiveDate;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct DbShift{
    pub id            : Uuid,
    pub shift_date    : NaiveDate,
    pub shift_order   : i16,
}
