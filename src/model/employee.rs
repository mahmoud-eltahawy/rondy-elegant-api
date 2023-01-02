use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Employee{
    id : i32,
    department_id : String,
    first_name: String,
    middle_name: String,
    last_name: String,
}
