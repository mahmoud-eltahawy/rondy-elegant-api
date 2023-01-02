use actix_web::web::Data;

use crate::{AppState, model::problem::Probelm};

pub async fn find_all_probelms(state : Data<AppState>) -> Vec<Probelm> {
    let query = "
        select
            id,
            shift_id,
            machine_id,
            begin_time,
            end_time
        from problem_detail
                ";
    match sqlx::query_as::<_, Probelm>(query).fetch_all(&state.db).await {
        Ok(problems) => problems,
        Err(_) => Vec::new()
    }
}
