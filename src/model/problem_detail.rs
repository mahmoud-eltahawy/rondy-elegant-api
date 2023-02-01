use chrono::NaiveTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{problem::Probelm, machine::Machine, employee::Employee, spare_part::SparePart};

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
  pub note              : Option<String>
}

#[derive(Serialize,Deserialize)]
pub struct ShiftProblem{
  pub id                : Option<Uuid>,
  pub shift_id          : Uuid,
  pub writer_id         : Employee,
  pub maintainer_id     : Employee,
  pub machine_id        : Machine,
  pub begin_time        : NaiveTime,
  pub end_time          : NaiveTime,
  pub problems_ids      : Vec<Probelm>,
  pub spare_parts_ids   : Option<Vec<SparePart>>,
  pub note              : Option<String>
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
