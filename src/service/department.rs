use rec::model::{
    department::{Department, UpdateDepartment},
    Environment, TableCrud, TableResponse, Wrapable,
};

use crate::{
    repo::{
        department::{
            delete, fetch_department_boss_id_by_id, fetch_department_by_id, save,
            set_department_boss, update_name,
        },
        employee::fetch_employee_department_id_by_id,
        permissions::{allow_permission, fetch_permissions_by_id, forbid_all_permissions},
    },
    AppState,
};

pub async fn crud(
    state: &AppState,
    crud: TableCrud<Department, UpdateDepartment>,
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
            let result = fetch_department_by_id(state, id).await?;
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
    env: Environment<UpdateDepartment>,
) -> Result<(), Box<dyn std::error::Error>> {
    let Environment {
        updater_id,
        time_stamp,
        target,
    } = env;
    match target {
        UpdateDepartment::ChangeBoss(employee_id) => {
            let department_id = fetch_employee_department_id_by_id(&state, &employee_id).await?;
            let Some(boss_id) = fetch_department_boss_id_by_id(&state, &department_id).await? else {
                set_department_boss(
                    state,
                    employee_id,
                    department_id,
                    (updater_id,time_stamp)
                ).await?;
                return Ok(());
            };
            let old_boss_permissions = fetch_permissions_by_id(state, &boss_id).await?;
            forbid_all_permissions(state, boss_id, (updater_id, time_stamp)).await?;
            for permission in old_boss_permissions {
                allow_permission(state, employee_id, permission, (updater_id, time_stamp)).await?;
            }
            set_department_boss(state, employee_id, department_id, (updater_id, time_stamp))
                .await?;
            Ok(())
        }
        UpdateDepartment::SetBoss(department_id, new_boss_id) => {
            set_department_boss(state, new_boss_id, department_id, (updater_id, time_stamp))
                .await?;
            Ok(())
        }
        UpdateDepartment::UpdateName(department_id, name) => {
            update_name(state, department_id, name, (updater_id, time_stamp)).await?;
            Ok(())
        }
    }
}
