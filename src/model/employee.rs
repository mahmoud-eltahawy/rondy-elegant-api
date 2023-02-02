use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
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

