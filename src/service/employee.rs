use crate::{
    repo::{
        employee::{
            delete, down, fetch_employee_by_id, save, up, update_department, update_password,
        },
        permissions::{allow_permission, forbid_all_permissions, forbid_permission},
    },
    AppState,
};
use rec::model::{
    employee::{Employee, UpdateEmployee},
    Environment, TableCrud, TableResponse, Wrapable,
};

pub async fn crud(
    state: &AppState,
    crud: TableCrud<Employee, UpdateEmployee>,
) -> Result<TableResponse, Box<dyn std::error::Error>> {
    match crud {
        TableCrud::Create(env) => {
            let Environment {
                updater_id,
                time_stamp,
                target,
            } = env;
            save(state, target, (updater_id, time_stamp)).await?;
            Ok(TableResponse::Done)
        }
        TableCrud::Read(id) => {
            let result = fetch_employee_by_id(state, id).await?;
            Ok(result.wrap())
        }
        TableCrud::Update(env) => {
            update(state, env).await?;
            Ok(TableResponse::Done)
        }
        TableCrud::Delete(env, _) => {
            let Environment {
                updater_id,
                time_stamp,
                target,
            } = env;
            delete(state, target, (updater_id, time_stamp)).await?;
            Ok(TableResponse::Done)
        }
    }
}

async fn update(
    state: &AppState,
    env: Environment<UpdateEmployee>,
) -> Result<(), Box<dyn std::error::Error>> {
    let Environment {
        updater_id,
        time_stamp,
        target,
    } = env;
    match target {
        UpdateEmployee::Down(id) => {
            down(state, id, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateEmployee::Up(id) => {
            up(state, id, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateEmployee::UpdateDepartment(employee_id, department_id) => {
            update_department(state, employee_id, department_id, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateEmployee::UpdatePassword(employee_id, password) => {
            update_password(state, employee_id, password, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateEmployee::ForbidAllPermissions(employee_id) => {
            forbid_all_permissions(state, employee_id, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateEmployee::ForbidPermission(employee_id, permission) => {
            forbid_permission(state, employee_id, permission, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateEmployee::AllowPermission(employee_id, permission) => {
            allow_permission(state, employee_id, permission, (updater_id, time_stamp)).await?;
            Ok(())
        }
    }
}
