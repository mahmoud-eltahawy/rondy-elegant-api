use chrono::NaiveDateTime;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::{
    crud_sync::{Cd, CdVersion, Table, UpdateVersion},
    model::{
        machine::{Machine, UpdateMachine},
        Update,
    },
};

use super::syncing::{record_cd_version, record_update_version};

pub async fn save(
    state: &AppState,
    machine: Machine,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    let Machine { id, name } = machine;
    query!(
        "
    INSERT INTO machine(id,name)
    VALUES($1,$2);",
        id,
        name
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        &state,
        CdVersion {
            cd: Cd::Create,
            target_table: Table::Machine,
            updater_id,
            time_stamp,
            target_id: id,
            version_number: 0,
        },
    )
    .await?;
    Ok(())
}

pub async fn update_name(
    state: &AppState,
    id: Uuid,
    name: String,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE machine SET
    name = $2
    WHERE id = $1;",
        id,
        name
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        state,
        UpdateVersion {
            target_id: id,
            updater_id,
            time_stamp,
            json: Update::Machine(UpdateMachine::UpdateName(id, name)),
            version_number: 0,
        },
    )
    .await?;
    Ok(())
}

pub async fn delete(
    state: &AppState,
    id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE machine SET
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
            target_table: Table::Machine,
            target_id: id,
            updater_id,
            time_stamp,
            version_number: 0,
        },
    )
    .await?;
    Ok(())
}

pub async fn fetch_machine_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Machine, Box<dyn std::error::Error>> {
    let row = query_as!(
        Machine,
        r#"
        select id,name
        from machine WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db);
    match row.await {
        Ok(machine) => Ok(machine),
        Err(err) => Err(err.into()),
    }
}
