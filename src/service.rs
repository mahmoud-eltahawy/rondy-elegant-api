use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use rec::model::{TableRequest, TableResponse};
use uuid::Uuid;

use std::{error::Error, time::Duration};

use actix_web::{get, Scope};
use rec::crud_sync::Version;
use settimeout::set_timeout;

use crate::{
    repo::{
        department_shift::find_shift_by_id,
        syncing::{get_cd_version, get_update_version, last_version},
    },
    AppState,
};

mod department;
mod employee;
mod machine;
mod problem;
mod shift;
mod shift_problem;
mod spare_part;

pub fn scope() -> Scope {
    web::scope("/app")
        .service(get_last_updates)
        .service(crud)
        .service(get_shift)
}

#[post("/")]
async fn crud(state: Data<AppState>, table: web::Json<TableRequest>) -> impl Responder {
    let state = &**state;
    match crud_helper(state, table.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::Ok().json(TableResponse::Err(err.to_string())),
    }
}
async fn crud_helper(
    state: &AppState,
    table: TableRequest,
) -> Result<TableResponse, Box<dyn Error>> {
    let result = match table {
        TableRequest::Department(varient) => department::crud(state, varient).await?,
        TableRequest::DepartmentShift(varient) => shift::crud(state, varient).await?,
        TableRequest::Employee(varient) => employee::crud(state, varient).await?,
        TableRequest::Machine(varient) => machine::crud(state, varient).await?,
        TableRequest::Problem(varient) => problem::crud(state, varient).await?,
        TableRequest::SparePart(varient) => spare_part::crud(state, varient).await?,
        TableRequest::ShiftProblem(varient) => shift_problem::crud(state, varient).await?,
    };
    Ok(result)
}

#[get("/{cd}/{update}")]
async fn get_last_updates(state: Data<AppState>, version: web::Path<(u64, u64)>) -> impl Responder {
    let (recieved_cd_version, recieved_update_version) = version.into_inner();
    let Ok((current_cd_version,current_update_version)) = last_version(&state.db).await else {
        return HttpResponse::InternalServerError().into();
    };

    let mut versions: Vec<Version> = Vec::new();

    if recieved_cd_version == current_cd_version
        && recieved_update_version == current_update_version
    {
        set_timeout(Duration::from_secs(1)).await;
        return HttpResponse::Ok().json(versions);
    }

    if recieved_cd_version > current_cd_version || recieved_update_version > current_update_version
    {
        return HttpResponse::InternalServerError().into();
    }

    if recieved_cd_version < current_cd_version {
        for v in recieved_cd_version + 1..=current_cd_version {
            if let Ok(version) = get_cd_version(&state.db, v).await {
                versions.push(Version::Cd(version))
            }
        }
    }

    if recieved_update_version < current_update_version {
        for v in recieved_update_version + 1..=current_update_version {
            if let Ok(version) = get_update_version(&state.db, v).await {
                versions.push(Version::Update(version))
            }
        }
    }

    HttpResponse::Ok().json(versions)
}

#[get("/{id}")]
async fn get_shift(state: Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();
    match find_shift_by_id(&*state, id).await {
        Ok(shift) => HttpResponse::Ok().json(shift),
        Err(_) => HttpResponse::NoContent().into(),
    }
}
