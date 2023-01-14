use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Serialize,Deserialize,FromRow)]
pub struct SparePart{
    pub id         : Uuid,
    pub name       : String,
}
