use chrono::{NaiveDateTime, NaiveTime};
use rec::{
    crud_sync::{Cd, CdVersion, Table, UpdateVersion},
    model::{
        note::Note,
        shift_problem::{ShiftProblem, UpdateShiftProblem},
        Update,
    },
};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;

use super::syncing::{record_cd_version, record_update_version};

pub async fn find_shift_problem_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<ShiftProblem, Box<dyn std::error::Error>> {
    let problem = query_as!(
        ShiftProblem,
        r#"
        select
            id,
            shift_id,
            maintainer_id,
            machine_id,
            begin_time,
            end_time
        from shift_problem
        WHERE id = $1;
        "#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(problem)
}

pub async fn save_shift_problem(
    state: &AppState,
    shift_problem: ShiftProblem,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    let ShiftProblem {
        id,
        shift_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time,
    } = shift_problem;
    query!(
        "
      INSERT INTO shift_problem(
          id,
          shift_id,
          maintainer_id,
          machine_id,
          begin_time,
          end_time)
      VALUES($1,$2,$3,$4,$5,$6);",
        id,
        shift_id,
        maintainer_id,
        machine_id,
        begin_time,
        end_time
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        &state,
        CdVersion {
            cd: Cd::Create,
            target_table: Table::ShiftProblem,
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: id,
        },
    )
    .await?;
    Ok(())
}

pub async fn update_shift_problem_maintainer(
    state: &AppState,
    shift_problem_id: Uuid,
    maintainer_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
      UPDATE shift_problem SET
      maintainer_id = $2
      WHERE id = $1;",
        shift_problem_id,
        maintainer_id,
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: shift_problem_id,
            time_stamp,
            updater_id,
            json: Update::ShiftProblem(UpdateShiftProblem::UpdateMaintainer(
                shift_problem_id,
                maintainer_id,
            )),
        },
    )
    .await?;
    Ok(())
}

pub async fn update_shift_problem_machine(
    state: &AppState,
    shift_problem_id: Uuid,
    machine_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
      UPDATE shift_problem SET
      machine_id = $2
      WHERE id = $1;",
        shift_problem_id,
        machine_id,
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: shift_problem_id,
            time_stamp,
            updater_id,
            json: Update::ShiftProblem(UpdateShiftProblem::UpdateMachine(
                shift_problem_id,
                machine_id,
            )),
        },
    )
    .await?;
    Ok(())
}

pub async fn update_shift_problem_begin_time(
    state: &AppState,
    shift_problem_id: Uuid,
    begin_time: NaiveTime,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
      UPDATE shift_problem SET
      begin_time = $2
      WHERE id = $1;",
        shift_problem_id,
        begin_time,
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: shift_problem_id,
            time_stamp,
            updater_id,
            json: Update::ShiftProblem(UpdateShiftProblem::UpdateBeginTime(
                shift_problem_id,
                begin_time,
            )),
        },
    )
    .await?;
    Ok(())
}

pub async fn update_shift_problem_end_time(
    state: &AppState,
    shift_problem_id: Uuid,
    end_time: NaiveTime,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
      UPDATE shift_problem SET
      end_time = $2
      WHERE id = $1;",
        shift_problem_id,
        end_time,
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: shift_problem_id,
            time_stamp,
            updater_id,
            json: Update::ShiftProblem(UpdateShiftProblem::UpdateEndTime(
                shift_problem_id,
                end_time,
            )),
        },
    )
    .await?;
    Ok(())
}

pub async fn delete_shift_problem(
    state: &AppState,
    shift_problem_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
      DELETE FROM shift_problem
      WHERE id = $1;",
        shift_problem_id
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        &state,
        CdVersion {
            cd: Cd::Delete,
            target_table: Table::ShiftProblem,
            version_number: 0,
            updater_id,
            time_stamp,
            target_id: shift_problem_id,
        },
    )
    .await?;
    Ok(())
}

pub async fn save_shift_problem_note(
    state: &AppState,
    id: Uuid,
    content: String,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    INSERT INTO shift_problem_note(id,content)
    VALUES($1,$2);",
        id,
        content
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: id,
            updater_id,
            time_stamp,
            json: Update::ShiftProblem(UpdateShiftProblem::AddNote(Note { id, content })),
        },
    )
    .await?;
    Ok(())
}

pub async fn update_shift_problem_note(
    state: &AppState,
    id: Uuid,
    content: String,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE shift_problem_note SET content = $2 WHERE id =$1;",
        id,
        content
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: id,
            updater_id,
            time_stamp,
            json: Update::ShiftProblem(UpdateShiftProblem::UpdateNote(Note { id, content })),
        },
    )
    .await?;
    Ok(())
}

pub async fn remove_shift_problem_note(
    state: &AppState,
    id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    DELETE FROM shift_problem_note WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            target_id: id,
            updater_id,
            time_stamp,
            json: Update::ShiftProblem(UpdateShiftProblem::DeleteNote(id)),
        },
    )
    .await?;
    Ok(())
}
