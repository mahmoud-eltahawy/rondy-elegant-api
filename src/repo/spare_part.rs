use std::error::Error;

use chrono::NaiveDateTime;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::{
    crud_sync::{Cd, CdVersion, Table, UpdateVersion},
    model::{
        spare_part::{SparePart, UpdateSparePart},
        Update,
    },
};

use super::syncing::{record_cd_version, record_update_version};

pub async fn fetch_spare_part_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<SparePart, Box<dyn Error>> {
    let part = query_as!(
        SparePart,
        r#"
        select id,name
        from spare_part WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(part)
}

pub async fn save(
    state: &AppState,
    part: SparePart,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    let SparePart { id, name } = part;
    query!(
        "
    INSERT INTO spare_part(id,name)
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
            target_table: Table::SparePart,
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: id,
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
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE spare_part
    SET name = $2
    WHERE id = $1;",
        id,
        name
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
            json: Update::SparePart(UpdateSparePart::UpdateName(id, name)),
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
    UPDATE spare_part SET
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
            target_table: Table::SparePart,
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: id,
        },
    )
    .await?;
    Ok(())
}
