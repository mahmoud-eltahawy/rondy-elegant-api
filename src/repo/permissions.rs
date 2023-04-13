use std::error::Error;

use chrono::NaiveDateTime;
use rec::{
    crud_sync::UpdateVersion,
    model::{employee::UpdateEmployee, permissions::PermissionName, Update},
};
use sqlx::query;
use uuid::Uuid;

use crate::AppState;

use super::syncing::record_update_version;

pub async fn fetch_permissions_by_id(
    state: &AppState,
    id: &Uuid,
) -> Result<Vec<PermissionName>, Box<dyn std::error::Error>> {
    let row = query!(
        r#"
        select permission from permissions WHERE employee_id = $1"#,
        id
    )
    .fetch_all(&state.db);
    match row.await {
        Ok(records) => Ok(records
            .into_iter()
            .flat_map(|r| PermissionName::try_from(r.permission))
            .collect()),
        Err(err) => Err(err.into()),
    }
}

pub async fn allow_permission(
    state: &AppState,
    id: Uuid,
    permission: PermissionName,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        r#"
        INSERT INTO permissions(employee_id,permission)
        VALUES($1,$2)
    "#,
        id,
        permission.stringify()
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
            json: Update::Employee(UpdateEmployee::AllowPermission(id, permission)),
        },
    )
    .await?;
    Ok(())
}

pub async fn forbid_permission(
    state: &AppState,
    employee_id: Uuid,
    permission: PermissionName,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        r#"
        DELETE FROM permissions WHERE employee_id = $1 AND permission = $2
    "#,
        employee_id,
        permission.stringify()
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: employee_id,
            updater_id,
            time_stamp,
            json: Update::Employee(UpdateEmployee::ForbidPermission(employee_id, permission)),
        },
    )
    .await?;
    Ok(())
}

pub async fn forbid_all_permissions(
    state: &AppState,
    id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        r#"
        DELETE FROM permissions WHERE employee_id = $1;
    "#,
        id
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
            json: Update::Employee(UpdateEmployee::ForbidAllPermissions(id)),
        },
    )
    .await?;
    Ok(())
}
