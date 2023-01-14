use actix_web::web::Data;

use crate::{AppState, model::spare_part::SparePart};

pub async fn find_all_spare_parts(state : Data<AppState>) -> Vec<SparePart> {
    let query = "
        select
            id,
            name
        from spare_part
                ";
    match sqlx::query_as::<_, SparePart>(query).fetch_all(&state.db).await {
        Ok(problems) => problems,
        Err(_) => Vec::new()
    }
}
