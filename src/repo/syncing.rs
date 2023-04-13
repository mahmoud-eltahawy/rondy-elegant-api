use rec::crud_sync::{Cd, CdVersion, Table, UpdateVersion};
use sqlx::{query, Pool, Postgres};

use std::error::Error;

use crate::AppState;

pub async fn record_cd_version(
    state: &AppState,
    cud_version: CdVersion,
) -> Result<(), Box<dyn Error>> {
    match cud_version.cd {
        Cd::Create => create_version(state, cud_version).await?,
        Cd::Delete => delete_version(state, cud_version).await?,
    }
    Ok(())
}

pub async fn record_update_version(
    state: &AppState,
    version: UpdateVersion,
) -> Result<(), Box<dyn Error>> {
    let UpdateVersion {
        target_id,
        json,
        time_stamp,
        updater_id,
        version_number: _,
    } = version;

    let json = serde_json::json!(json);

    query!(
        "
    INSERT INTO update_version(target_id,updater_id,time_stamp,json)
    VALUES($1,$2,$3,$4);
    ",
        target_id,
        updater_id,
        time_stamp,
        json
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

async fn delete_version(state: &AppState, cud_version: CdVersion) -> Result<(), Box<dyn Error>> {
    let target_id = cud_version.target_id;
    query!(
        "
    DELETE FROM cd_version
    WHERE target_id = $1;
    ",
        target_id,
    )
    .execute(&state.db)
    .await?;
    query!(
        "
     DELETE FROM update_version
     WHERE target_id = $1;
    ",
        target_id,
    )
    .execute(&state.db)
    .await?;
    create_version(state, cud_version).await?;
    Ok(())
}

async fn create_version(state: &AppState, cd_version: CdVersion) -> Result<(), Box<dyn Error>> {
    let CdVersion {
        target_id,
        updater_id,
        time_stamp,
        cd,
        target_table,
        version_number: _,
    } = cd_version;
    query!(
        "
    INSERT INTO cd_version(
        target_id,
        updater_id,
        time_stamp,
        target_table,
        cd
    )
    VALUES($1,$2,$3,$4,$5);",
        target_id,
        updater_id,
        time_stamp,
        target_table.stringify(),
        cd.stringify()
    )
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn last_version(state: &Pool<Postgres>) -> Result<(u64, u64), Box<dyn Error>> {
    let cd_row = query!(
        "
  SELECT COALESCE(MAX(version_number),0) as current_version FROM cd_version;
  "
    )
    .fetch_one(state);
    let update_row = query!(
        "
  SELECT COALESCE(MAX(version_number),0) as current_version FROM update_version;
  "
    )
    .fetch_one(state);
    match (cd_row.await, update_row.await) {
        (Ok(cd), Ok(update)) => match (cd.current_version, update.current_version) {
            (Some(cd), Some(update)) => Ok((cd as u64, update as u64)),
            _ => Err("error".into()),
        },
        _ => Err("error".into()),
    }
}

pub async fn get_cd_version(
    state: &Pool<Postgres>,
    version: u64,
) -> Result<CdVersion, Box<dyn Error>> {
    let row = query!(
        "
  SELECT
    version_number,
    target_id,
    updater_id,
    time_stamp,
    target_table,
    cd
  FROM cd_version WHERE version_number = $1;
  ",
        Some(version as i64)
    )
    .fetch_one(state);
    match row.await {
        Ok(record) => match (
            Table::try_from(record.target_table),
            Cd::try_from(record.cd),
        ) {
            (Ok(target_table), Ok(cd)) => Ok(CdVersion {
                version_number: record.version_number as u64,
                target_id: record.target_id,
                target_table,
                cd,
                time_stamp: record.time_stamp,
                updater_id: record.updater_id,
            }),
            _ => Err("".into()),
        },
        Err(err) => Err(err.into()),
    }
}

pub async fn get_update_version(
    state: &Pool<Postgres>,
    version: u64,
) -> Result<UpdateVersion, Box<dyn Error>> {
    let record = query!(
        "
  SELECT
    version_number,
    updater_id,
    target_id,
    time_stamp,
    json
  FROM update_version WHERE version_number = $1;
  ",
        Some(version as i64)
    )
    .fetch_one(state)
    .await?;
    Ok(UpdateVersion {
        version_number: record.version_number as u64,
        target_id: record.target_id,
        updater_id: record.updater_id,
        time_stamp: record.time_stamp,
        json: serde_json::from_value(record.json)?,
    })
}
