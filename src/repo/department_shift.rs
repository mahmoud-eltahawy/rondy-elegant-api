use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{query, query_as};
use uuid::Uuid;

use std::error::Error;

use crate::AppState;
use rec::{
    crud_sync::{Cd, CdVersion, Table, UpdateVersion},
    model::{
        note::{Note, ShiftNote},
        shift::{DepartmentShift, Shift, ShiftOrder, UpdateDepartmentShift},
        Update,
    },
    timer::{get_current_date, get_current_order, get_relative_now},
};

use super::syncing::{record_cd_version, record_update_version};

pub async fn find_shift_by_id(state: &AppState, id: Uuid) -> Result<Shift, Box<dyn Error>> {
    let record = query!(
        r#"
        select
            id,
            shift_order,
            shift_date
        from shift where id = $1
    "#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    match ShiftOrder::try_from(record.shift_order) {
        Ok(shift_order) => Ok(Shift {
            id: record.id,
            shift_date: record.shift_date,
            shift_order,
        }),
        Err(err) => Err(err.into()),
    }
}

pub async fn remove_department_shift_note(
    state: &AppState,
    shift_id: Uuid,
    note_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    DELETE FROM shift_note WHERE id = $1",
        note_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        state,
        UpdateVersion {
            version_number: 0,
            target_id: shift_id,
            time_stamp,
            updater_id,
            json: Update::DepartmentShift(UpdateDepartmentShift::DeleteNote(shift_id, note_id)),
        },
    )
    .await?;
    Ok(())
}

pub async fn update_department_shift_note_content(
    state: &AppState,
    id: Uuid,
    content: String,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE shift_note SET content = $2 WHERE id =$1;",
        id,
        content
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        state,
        UpdateVersion {
            version_number: 0,
            target_id: id,
            time_stamp,
            updater_id,
            json: Update::DepartmentShift(UpdateDepartmentShift::UpdateNote(Note { id, content })),
        },
    )
    .await?;
    Ok(())
}

pub async fn save_department_shift_note(
    state: &AppState,
    note: ShiftNote,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    let ShiftNote {
        id,
        shift_id,
        content,
    } = &note;
    query!(
        "
    INSERT INTO shift_note(id,shift_id,content)
    VALUES($1,$2,$3);",
        id,
        shift_id,
        content
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        state,
        UpdateVersion {
            version_number: 0,
            target_id: *shift_id,
            time_stamp,
            updater_id,
            json: Update::DepartmentShift(UpdateDepartmentShift::SaveNote(note)),
        },
    )
    .await?;
    Ok(())
}

pub async fn find_department_shift_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<DepartmentShift, Box<dyn Error>> {
    let result = query_as!(
        DepartmentShift,
        r#"
        select id,shift_id,department_id from department_shift where id = $1
    "#,
        id
    )
    .fetch_one(&state.db)
    .await?;
    Ok(result)
}

pub async fn save_department_shift(
    state: &AppState,
    department_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let id = Uuid::new_v4();
    let shift_id = save_or_get_shift_id(state, &env).await?;
    let (updater_id, time_stamp) = env;
    query!(
        "
        INSERT INTO department_shift(id,shift_id,department_id)
        VALUES($1,$2,$3)
    ",
        id,
        shift_id,
        department_id
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        state,
        CdVersion {
            cd: Cd::Create,
            target_table: Table::DepartmentShift,
            time_stamp,
            updater_id,
            version_number: 0,
            target_id: id,
        },
    )
    .await?;
    Ok(())
}

pub async fn remove_department_shift(
    state: &AppState,
    id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "UPDATE department_shift SET deleted = TRUE WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        state,
        CdVersion {
            cd: Cd::Delete,
            target_table: Table::DepartmentShift,
            version_number: 0,
            updater_id,
            time_stamp,
            target_id: id,
        },
    )
    .await?;
    Ok(())
}

pub async fn save_department_shift_employee(
    state: &AppState,
    department_shift_id: Uuid,
    employee_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
        INSERT INTO department_shift_employee(department_shift_id,employee_id)
        VALUES($1,$2)
    ",
        department_shift_id,
        employee_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        state,
        UpdateVersion {
            version_number: 0,
            target_id: department_shift_id,
            time_stamp,
            updater_id,
            json: Update::DepartmentShift(UpdateDepartmentShift::SaveShiftEmployee(
                department_shift_id,
                employee_id,
            )),
        },
    )
    .await?;
    Ok(())
}

pub async fn remove_department_shift_employee(
    state: &AppState,
    department_shift_id: Uuid,
    employee_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
        DELETE FROM department_shift_employee
        WHERE department_shift_id = $1
        AND employee_id = $2;
    ",
        department_shift_id,
        employee_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        state,
        UpdateVersion {
            version_number: 0,
            target_id: department_shift_id,
            time_stamp,
            updater_id,
            json: Update::DepartmentShift(UpdateDepartmentShift::DeleteShiftEmployee(
                department_shift_id,
                employee_id,
            )),
        },
    )
    .await?;
    Ok(())
}

async fn save_or_get_shift_id(
    state: &AppState,
    env: &(Uuid, NaiveDateTime),
) -> Result<Uuid, Box<dyn Error>> {
    let now = get_relative_now();
    let date = get_current_date(now);
    let order = get_current_order(now);
    let Some(date) = date else {
       return Err("date is null".into())
    };
    match find_shift_id_by_date_and_order(state, date, &order).await {
        Ok(id) => Ok(id),
        Err(_) => {
            let id = Uuid::new_v4();
            save_shift(
                state,
                Shift {
                    id,
                    shift_date: date,
                    shift_order: order,
                },
                *env,
            )
            .await?;
            Ok(id)
        }
    }
}

async fn find_shift_id_by_date_and_order(
    state: &AppState,
    date: NaiveDate,
    order: &ShiftOrder,
) -> Result<Uuid, Box<dyn Error>> {
    let record = query!(
        r#"
      select
          id,
          shift_order,
          shift_date
      from shift where shift_date = $1 and shift_order = $2
  "#,
        date,
        order.stringify()
    )
    .fetch_one(&state.db)
    .await?;
    Ok(record.id)
}

async fn save_shift(
    state: &AppState,
    shift: Shift,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn Error>> {
    let (updater_id, time_stamp) = env;
    let Shift {
        id,
        shift_date,
        shift_order,
    } = shift;
    query!(
        "
      INSERT INTO shift(id,shift_order,shift_date)
      VALUES($1,$2,$3)
  ",
        id,
        shift_order.stringify(),
        shift_date
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        state,
        CdVersion {
            cd: Cd::Create,
            target_table: Table::Shift,
            version_number: 0,
            time_stamp,
            updater_id,
            target_id: id,
        },
    )
    .await?;
    Ok(())
}
