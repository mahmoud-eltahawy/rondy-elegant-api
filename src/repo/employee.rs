use bcrypt::BcryptResult;
use chrono::NaiveDateTime;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::AppState;
use rec::{
    crud_sync::{Cd, CdVersion, Table, UpdateVersion},
    model::{
        employee::{Employee, UpdateEmployee},
        Update,
    },
};

use super::syncing::{record_cd_version, record_update_version};

pub async fn save(
    state: &AppState,
    employee: Employee,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    let Employee {
        id,
        department_id,
        card_id,
        position,
        first_name,
        middle_name,
        last_name,
        password,
    } = employee;
    let password = hash_password(password)?;
    query!(
        "
    INSERT INTO employee(
    id,department_id,position,
    first_name,middle_name,last_name,
    card_id,password)
    VALUES($1,$2,$3,$4,$5,$6,$7,$8);",
        id,
        department_id,
        position,
        first_name,
        middle_name,
        last_name,
        card_id,
        password
    )
    .execute(&state.db)
    .await?;
    record_cd_version(
        &state,
        CdVersion {
            cd: Cd::Create,
            target_table: Table::Employee,
            time_stamp,
            updater_id,
            target_id: employee.id,
            version_number: 0,
        },
    )
    .await?;
    Ok(())
}

pub async fn down(
    state: &AppState,
    id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE employee SET
    position = 'USER'
    WHERE id = $1;",
        id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            target_id: id,
            time_stamp,
            updater_id,
            json: Update::Employee(UpdateEmployee::Down(id)),
            version_number: 0,
        },
    )
    .await?;
    Ok(())
}

pub async fn update_password(
    state: &AppState,
    employee_id: Uuid,
    password: String,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    let password = hash_password(password)?;
    query!(
        "
    UPDATE employee SET
    password = $2
    WHERE id = $1;",
        employee_id,
        password
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            target_id: employee_id,
            time_stamp,
            updater_id,
            json: Update::Employee(UpdateEmployee::UpdatePassword(employee_id, password)),
            version_number: 0,
        },
    )
    .await?;
    Ok(())
}

pub async fn update_department(
    state: &AppState,
    employee_id: Uuid,
    department_id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE employee SET
    department_id = $2
    WHERE id = $1;",
        employee_id,
        department_id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            target_id: employee_id,
            time_stamp,
            updater_id,
            json: Update::Employee(UpdateEmployee::UpdateDepartment(employee_id, department_id)),
            version_number: 0,
        },
    )
    .await?;
    Ok(())
}

pub async fn up(
    state: &AppState,
    id: Uuid,
    env: (Uuid, NaiveDateTime),
) -> Result<(), Box<dyn std::error::Error>> {
    let (updater_id, time_stamp) = env;
    query!(
        "
    UPDATE employee SET
    position = 'SUPER_USER'
    WHERE id = $1;",
        id
    )
    .execute(&state.db)
    .await?;
    record_update_version(
        &state,
        UpdateVersion {
            target_id: id,
            time_stamp,
            updater_id,
            json: Update::Employee(UpdateEmployee::Up(id)),
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
    UPDATE employee SET
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
            time_stamp,
            updater_id,
            target_table: Table::Employee,
            target_id: id,
            version_number: 0,
        },
    )
    .await?;
    Ok(())
}

pub async fn fetch_employee_by_id(
    state: &AppState,
    id: Uuid,
) -> Result<Employee, Box<dyn std::error::Error>> {
    let row = query_as!(
        Employee,
        r#"select
      id,
      department_id,
      position,
      first_name,
      middle_name,
      last_name,
      card_id,
      password
 from employee where id = $1"#,
        id
    )
    .fetch_one(&state.db);
    match row.await {
        Ok(emp) => Ok(emp),
        Err(err) => Err(err.into()),
    }
}

pub async fn fetch_employee_department_id_by_id(
    state: &AppState,
    id: &Uuid,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let row = query!(
        r#"
    select department_id
    from employee where id = $1"#,
        id
    )
    .fetch_one(&state.db);
    match row.await {
        Ok(emp) => Ok(emp.department_id),
        Err(err) => Err(err.into()),
    }
}

fn hash_password(password: String) -> BcryptResult<String> {
    bcrypt::hash(password, 8)
}
