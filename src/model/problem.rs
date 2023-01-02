use chrono::NaiveTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Serialize,Deserialize,FromRow)]
pub struct Probelm{
    pub id : Uuid,
    pub shift_id : Uuid,
    pub machine_id : Uuid,
    pub begin_time : NaiveTime,
    pub end_time : NaiveTime
}
