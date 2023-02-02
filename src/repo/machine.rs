use actix_web::web::Data;
use sqlx::query_as;
use uuid::Uuid;

use crate::{AppState, model::machine::Machine};

pub async fn find_all_machines(state : Data<AppState>) -> Vec<Machine> {
    let query = "
        select
            id,
            name
        from machine
                ";
    match sqlx::query_as::<_, Machine>(query).fetch_all(&state.db).await {
        Ok(problems) => problems,
        Err(_) => Vec::new()
    }
}

pub async fn fetch_machine_by_id(state : Data<AppState>,id : Uuid) -> Option<Machine> {
  let row = query_as!(Machine,r#"
        select id,name
        from machine WHERE id = $1"#,id)
    .fetch_one(&state.db);
  match row.await {
    Ok(machine) =>Some(machine),
    Err(_) => None
  }
}
