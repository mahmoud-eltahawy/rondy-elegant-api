use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

pub static POSITIONS : [&str;3] = ["ADMIN","SUPER_USER","USER"];

#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct Employee{
    pub id              : Option<Uuid>,
    pub department_id   : Uuid,
    pub position        : String,
    pub first_name      : String,
    pub middle_name     : String,
    pub last_name       : String,
    pub card_id         : i16,
    pub password        : String
}

