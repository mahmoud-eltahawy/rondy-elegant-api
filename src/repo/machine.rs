use actix_web::web::Data;

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
