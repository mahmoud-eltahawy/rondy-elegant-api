use std::error::Error;

use chrono::NaiveDateTime;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::{
    crud_sync::{Cd, CdVersion, Table, UpdateVersion},
    model::{
        problem::{Problem, UpdateProblem},
        Update,
    },
};

use super::syncing::{record_cd_version, record_update_version};

pub async fn fetch_problem_by_id(state: &AppState, id: Uuid) -> Result<Problem, Box<dyn Error>> {
    let machine = query_as!(
        Problem,
        r#"
        select id,department_id,title ,description
        from problem WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(machine)
}

pub async fn save(
    state: &AppState,
    problem: Problem,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    let Problem {
        id,
        department_id,
        title,
        description,
    } = problem;
    query!(
        "
    INSERT INTO problem(id,department_id,title,description)
    VALUES($1,$2,$3,$4);",
        id,
        department_id,
        title,
        description
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        &state,
        CdVersion {
            cd: Cd::Create,
            target_table: Table::Problem,
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: id,
        },
    )
    .await?;
    Ok(())
}

pub async fn update_title(
    state: &AppState,
    id: Uuid,
    title: String,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE problem SET
    title         = $2
    WHERE id      = $1;",
        id,
        title,
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: id,
            time_stamp,
            updater_id,
            json: Update::Problem(UpdateProblem::UpdateTitle(id, title)),
        },
    )
    .await?;
    Ok(())
}

pub async fn update_description(
    state: &AppState,
    id: Uuid,
    description: String,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE problem SET
    description   = $2
    WHERE id      = $1;",
        id,
        description,
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: id,
            time_stamp,
            updater_id,
            json: Update::Problem(UpdateProblem::UpdateDescription(id, description)),
        },
    )
    .await?;
    Ok(())
}

pub async fn delete(
    state: &AppState,
    id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE problem SET
    deleted = TRUE
    WHERE id = $1;",
        id
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        &state,
        CdVersion {
            cd: Cd::Delete,
            target_table: Table::Problem,
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: id,
        },
    )
    .await?;
    Ok(())
}
