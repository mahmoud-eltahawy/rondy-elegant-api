use std::error::Error;

use chrono::NaiveDateTime;
use rec::{
    crud_sync::UpdateVersion,
    model::{shift_problem::UpdateShiftProblem, Update},
};
use sqlx::query;
use uuid::Uuid;

use crate::{repo::syncing::record_update_version, AppState};

pub async fn save_problem_to_shift_problem(
    state: &AppState,
    problem_id: Uuid,
    shift_problem_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    INSERT INTO shift_problem_problem(
        shift_problem_id,
        problem_id)
    VALUES($1,$2);",
        shift_problem_id,
        problem_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: shift_problem_id,
            json: Update::ShiftProblem(UpdateShiftProblem::AddProblem(
                shift_problem_id,
                problem_id,
            )),
        },
    )
    .await?;
    Ok(())
}

pub async fn remove_problem_from_shift_problem(
    state: &AppState,
    problem_id: Uuid,
    shift_problem_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    DELETE FROM shift_problem_problem
    WHERE shift_problem_id = $1 AND problem_id = $2;",
        shift_problem_id,
        problem_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: shift_problem_id,
            json: Update::ShiftProblem(UpdateShiftProblem::DeleteProblem(
                shift_problem_id,
                problem_id,
            )),
        },
    )
    .await?;
    Ok(())
}

pub async fn save_spare_part_to_shift_problem(
    state: &AppState,
    part_id: Uuid,
    shift_problem_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    INSERT INTO shift_problem_spare_part(
        shift_problem_id,
        spare_part_id)
    VALUES($1,$2);",
        shift_problem_id,
        part_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: shift_problem_id,
            json: Update::ShiftProblem(UpdateShiftProblem::AddSparePart(shift_problem_id, part_id)),
        },
    )
    .await?;
    Ok(())
}

pub async fn remove_spare_part_from_shift_problem(
    state: &AppState,
    shift_problem_id: Uuid,
    part_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    DELETE FROM shift_problem_spare_part
    WHERE shift_problem_id = $1 AND spare_part_id = $2;",
        shift_problem_id,
        part_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: shift_problem_id,
            json: Update::ShiftProblem(UpdateShiftProblem::DeleteSparePart(
                shift_problem_id,
                part_id,
            )),
        },
    )
    .await?;
    Ok(())
}
