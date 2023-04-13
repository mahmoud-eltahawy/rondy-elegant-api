use chrono::NaiveDateTime;
use rec::{
    crud_sync::{Cd, CdVersion, Table, UpdateVersion},
    model::{
        department::{Department, UpdateDepartment},
        Update,
    },
};
use sqlx::{query, query_as};
use std::error::Error;
use uuid::Uuid;

use crate::AppState;

use super::syncing::{record_cd_version, record_update_version};

pub async fn fetch_department_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Department, Box<dyn Error>> {
    let dep = query_as!(
        Department,
        r#"
        select id,boss_id,name from department WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(dep)
}

pub async fn fetch_department_boss_id_by_id(
    state: &AppState,
    id: &Uuid,
) -> Result<Option<Uuid>, Box<dyn Error>> {
    let record = query!(
        r#"
        select boss_id
        from department WHERE id = $1"#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(record.boss_id)
}

pub async fn save(
    state: &AppState,
    department: Department,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let Department { id, boss_id, name } = department;
    let (updater_id, time_stamp) = env;
    query!(
        "
    INSERT INTO department(id,boss_id,name)
    VALUES($1,$2,$3);",
        id,
        boss_id,
        name
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        &state,
        CdVersion {
            cd: Cd::Create,
            time_stamp,
            updater_id,
            target_table: Table::Department,
            version_number: 0,
            target_id: id,
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
    UPDATE department SET
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
            target_table: Table::Department,
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
    UPDATE department SET
    name            = $2
    WHERE id        = $1;",
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
            json: Update::Department(UpdateDepartment::UpdateName(id, name)),
        },
    )
    .await?;
    Ok(())
}

pub async fn set_department_boss(
    state: &AppState,
    employee_id: Uuid,
    department_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        r#"
    UPDATE department SET boss_id = $1
    WHERE id = $2;
  "#,
        employee_id,
        department_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: department_id,
            time_stamp,
            updater_id,
            json: Update::Department(UpdateDepartment::SetBoss(department_id, employee_id)),
        },
    )
    .await?;
    Ok(())
}
