use rec::model::{
    note::Note,
    shift_problem::{ShiftProblem, UpdateShiftProblem},
    Environment, TableCrud, TableResponse, Wrapable,
};

use crate::{
    repo::{
        relations::shift_problem::{
            remove_problem_from_shift_problem, remove_spare_part_from_shift_problem,
            save_problem_to_shift_problem, save_spare_part_to_shift_problem,
        },
        shift_problem::{
            delete_shift_problem, find_shift_problem_by_id, remove_shift_problem_note,
            save_shift_problem, save_shift_problem_note, update_shift_problem_begin_time,
            update_shift_problem_end_time, update_shift_problem_machine,
            update_shift_problem_maintainer, update_shift_problem_note,
        },
    },
    AppState,
};

pub async fn crud(
    state: &AppState,
    varient: TableCrud<ShiftProblem, UpdateShiftProblem>,
) -> Result<TableResponse, Box<dyn std::error::Error>> {
    match varient {
        TableCrud::Read(id) => {
            let result = find_shift_problem_by_id(&state, id).await?;
            Ok(result.wrap())
        }
        TableCrud::Delete(env, _) => {
            let Environment {
                updater_id,
                time_stamp,
                target,
            } = env;
            delete_shift_problem(&state, target, (updater_id, time_stamp)).await?;
            Ok(TableResponse::Done)
        }
        TableCrud::Create(env) => {
            let Environment {
                updater_id,
                time_stamp,
                target,
            } = env;
            save_shift_problem(&state, target, (updater_id, time_stamp)).await?;
            Ok(TableResponse::Done)
        }
        TableCrud::Update(env) => {
            update(state, env).await?;
            Ok(TableResponse::Done)
        }
    }
}

async fn update(
    state: &AppState,
    env: Environment<UpdateShiftProblem>,
) -> Result<(), Box<dyn std::error::Error>> {
    let Environment {
        updater_id,
        time_stamp,
        target,
    } = env;
    match target {
        UpdateShiftProblem::AddProblem(shift_problem_id, problem_id) => {
            save_problem_to_shift_problem(
                &state,
                problem_id,
                shift_problem_id,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateShiftProblem::DeleteProblem(shift_problem_id, problem_id) => {
            remove_problem_from_shift_problem(
                &state,
                problem_id,
                shift_problem_id,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateShiftProblem::AddSparePart(shift_problem_id, spare_part_id) => {
            save_spare_part_to_shift_problem(
                &state,
                spare_part_id,
                shift_problem_id,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateShiftProblem::DeleteSparePart(shift_problem_id, spare_part_id) => {
            remove_spare_part_from_shift_problem(
                &state,
                shift_problem_id,
                spare_part_id,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateShiftProblem::UpdateBeginTime(shift_problem_id, begin_time) => {
            update_shift_problem_begin_time(
                state,
                shift_problem_id,
                begin_time,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateShiftProblem::UpdateEndTime(shift_problem_id, end_time) => {
            update_shift_problem_end_time(
                state,
                shift_problem_id,
                end_time,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateShiftProblem::UpdateMachine(shift_problem_id, machine_id) => {
            update_shift_problem_machine(
                state,
                shift_problem_id,
                machine_id,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateShiftProblem::UpdateMaintainer(shift_problem_id, maintainer_id) => {
            update_shift_problem_maintainer(
                state,
                shift_problem_id,
                maintainer_id,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateShiftProblem::AddNote(note) => {
            let Note { id, content } = note;
            save_shift_problem_note(state, id, content, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateShiftProblem::DeleteNote(shift_problem_id) => {
            remove_shift_problem_note(state, shift_problem_id, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateShiftProblem::UpdateNote(note) => {
            let Note { id, content } = note;
            update_shift_problem_note(state, id, content, (updater_id, time_stamp)).await?;
            Ok(())
        }
    }
}
