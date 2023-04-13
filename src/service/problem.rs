use crate::{
    repo::problem::{delete, fetch_problem_by_id, save, update_description, update_title},
    AppState,
};
use rec::model::{
    problem::{Problem, UpdateProblem},
    Environment, TableCrud, TableResponse, Wrapable,
};

pub async fn crud(
    state: &AppState,
    varient: TableCrud<Problem, UpdateProblem>,
) -> Result<TableResponse, Box<dyn std::error::Error>> {
    match varient {
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
            let result = fetch_problem_by_id(state, id).await?;
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
    env: Environment<UpdateProblem>,
) -> Result<(), Box<dyn std::error::Error>> {
    let Environment {
        updater_id,
        time_stamp,
        target,
    } = env;
    match target {
        UpdateProblem::UpdateTitle(id, title) => {
            update_title(state, id, title, (updater_id, time_stamp)).await?;
            Ok(())
        }
        UpdateProblem::UpdateDescription(id, description) => {
            update_description(state, id, description, (updater_id, time_stamp)).await?;
            Ok(())
        }
    }
}
