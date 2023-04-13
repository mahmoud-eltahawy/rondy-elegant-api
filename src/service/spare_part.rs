use rec::model::{
    spare_part::{SparePart, UpdateSparePart},
    Environment, TableCrud, TableResponse, Wrapable,
};

use crate::{
    repo::spare_part::{delete, fetch_spare_part_by_id, save, update_name},
    AppState,
};

pub async fn crud(
    state: &AppState,
    varient: TableCrud<SparePart, UpdateSparePart>,
) -> Result<TableResponse, Box<dyn std::error::Error>> {
    match varient {
        TableCrud::Read(id) => {
            let result = fetch_spare_part_by_id(state, id).await?;
            Ok(result.wrap())
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
        TableCrud::Create(env) => {
            let Environment {
                updater_id,
                time_stamp,
                target,
            } = env;
            save(state, target, (updater_id, time_stamp)).await?;
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
    env: Environment<UpdateSparePart>,
) -> Result<(), Box<dyn std::error::Error>> {
    let Environment {
        updater_id,
        time_stamp,
        target,
    } = env;
    match target {
        UpdateSparePart::UpdateName(id, name) => {
            update_name(state, id, name, (updater_id, time_stamp)).await?;
            Ok(())
        }
    }
}
