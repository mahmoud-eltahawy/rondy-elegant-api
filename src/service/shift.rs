use crate::{
    repo::department_shift::{
        find_department_shift_by_id, remove_department_shift, remove_department_shift_employee,
        remove_department_shift_note, save_department_shift, save_department_shift_employee,
        save_department_shift_note, update_department_shift_note_content,
    },
    AppState,
};
use rec::model::{
    note::Note,
    shift::{DepartmentShift, UpdateDepartmentShift},
    Environment, TableCrud, TableResponse, Wrapable,
};

pub async fn crud(
    state: &AppState,
    crud: TableCrud<DepartmentShift, UpdateDepartmentShift>,
) -> Result<TableResponse, Box<dyn std::error::Error>> {
    match crud {
        TableCrud::Create(env) => {
            let Environment {
                updater_id,
                time_stamp,
                target,
            } = env;
            let DepartmentShift {
                id: _,
                shift_id: _,
                department_id,
            } = target;
            save_department_shift(state, department_id, (updater_id, time_stamp)).await?;
            Ok(TableResponse::Done)
        }
        TableCrud::Read(shift_id) => {
            let shift = find_department_shift_by_id(state, shift_id).await?;
            Ok(shift.wrap())
        }
        TableCrud::Update(env) => {
            update_department_shift(state, env).await?;
            Ok(TableResponse::Done)
        }
        TableCrud::Delete(env, _) => {
            let Environment {
                updater_id,
                time_stamp,
                target,
            } = env;
            remove_department_shift(&state, target, (updater_id, time_stamp)).await?;
            Ok(TableResponse::Done)
        }
    }
}

async fn update_department_shift(
    state: &AppState,
    env: Environment<UpdateDepartmentShift>,
) -> Result<(), Box<dyn std::error::Error>> {
    let Environment {
        updater_id,
        time_stamp,
        target,
    } = env;
    match target {
        UpdateDepartmentShift::DeleteShiftEmployee(shift_id, employee_id) => {
            remove_department_shift_employee(
                &state,
                shift_id,
                employee_id,
                (updater_id, time_stamp),
            )
            .await?;
            Ok(())
        }
        UpdateDepartmentShift::SaveShiftEmployee(shift_id, employee_id) => {
            save_department_shift_employee(&state, shift_id, employee_id, (updater_id, time_stamp))
                .await?;
            Ok(())
        }
        UpdateDepartmentShift::SaveNote(note) => {
            save_department_shift_note(state, note, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateDepartmentShift::DeleteNote(shift_id, note_id) => {
            remove_department_shift_note(state, shift_id, note_id, (updater_id, time_stamp))
                .await?;
            Ok(())
        }
        UpdateDepartmentShift::UpdateNote(note) => {
            let Note { id, content } = note;
            update_department_shift_note_content(state, id, content, (updater_id, time_stamp))
                .await?;
            Ok(())
        }
    }
}
