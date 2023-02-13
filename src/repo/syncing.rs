use actix_web::web::Data;
use sqlx::{error::Error,query, Pool, Postgres, query_as};
use rec::crud_sync::{
  CudVersion,
  DbCudVersion,
};

use crate::AppState;

pub async fn record_version(state : &Data<AppState>,cud_version : CudVersion) -> Result<CudVersion,Error> {
  let CudVersion{target_id,cud,other_target_id,target_table,version_number:_} = cud_version;
  let row = query!("
    INSERT INTO cud_version(
        target_id,
        other_target_id,
        target_table,
        cud
    )
    VALUES($1,$2,$3,$4) RETURNING version_number;",
                   target_id,
                   other_target_id,
                   target_table.clone() as i16,
                   cud.clone() as i16
  ).fetch_one(&state.db);
  match row.await {
    Ok(num) => Ok(CudVersion {
      version_number: num.version_number.try_into().unwrap(),
      target_id,
      other_target_id,
      target_table,
      cud
    }),
    Err(err) => Err(err)
  }
}

pub async fn last_version(state : &Pool<Postgres>) -> Result<u64,Error> {
  let row = query!("
  SELECT MAX(version_number) as current_version FROM cud_version;
  ").fetch_one(state);
  match row.await {
    Ok(v) => {
      match v.current_version {
        Some(cv) => Ok(cv as u64),
        None => Err(sqlx::Error::PoolClosed)
      }
    },
    Err(err) => Err(err)
  }
}

pub async fn get_version(state : &Pool<Postgres>,version : u64) -> Result<CudVersion,Error> {
  let row = query_as!(DbCudVersion,"
  SELECT
    version_number,
    target_id,
    other_target_id,
    target_table,
    cud
  FROM cud_version WHERE version_number = $1;
  ",Some(version as i64)).fetch_one(state);
  match row.await {
    Ok(v) => Ok(CudVersion::get(v)),
    Err(err) => Err(err)
  }
}
