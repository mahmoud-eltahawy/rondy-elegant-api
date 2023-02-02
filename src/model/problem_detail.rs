use chrono::NaiveTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Serialize,Deserialize,FromRow)]
pub struct MinimamlShiftProblem{
  pub id                : Option<Uuid>,
  pub shift_id          : Uuid,
  pub writer_id         : Uuid,
  pub maintainer_id     : Uuid,
  pub machine_id        : Uuid,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
  pub problems_ids      : Vec<Uuid>,
  pub spare_parts_ids   : Option<Vec<Uuid>>,
  pub note              : Option<Note>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Note{
  pub id   : Uuid,
  pub content : String
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct DbShiftProblem{
  pub id                : Option<Uuid>,
  pub shift_id          : Uuid,
  pub writer_id         : Uuid,
  pub maintainer_id     : Uuid,
  pub machine_id        : Uuid,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
}
