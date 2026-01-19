use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
use thiserror::Error;
use uuid::Uuid;

use crate::schema::packages;

#[derive(Debug, Clone)]
pub struct StreamPackage {
    pub seq: i64,
    pub event_type: String,
    pub hash: Vec<u8>,
    pub prev_hash: Option<Vec<u8>>,
    pub value: strata::value::Value,
}

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("db error: {0}")]
    Db(DbErr),
    #[error("decode error: {0:?}")]
    Decode(strata::error::DecodeError),
}

impl From<DbErr> for ReadError {
    fn from(err: DbErr) -> Self {
        Self::Db(err)
    }
}

impl From<strata::error::DecodeError> for ReadError {
    fn from(err: strata::error::DecodeError) -> Self {
        Self::Decode(err)
    }
}

pub async fn read_stream_packages(
    db: &sea_orm::DatabaseConnection,
    stream_id: Uuid,
) -> Result<Vec<StreamPackage>, ReadError> {
    let rows = packages::Entity::find()
        .filter(packages::Column::StreamId.eq(stream_id))
        .order_by_asc(packages::Column::Seq)
        .all(db)
        .await?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let value = strata::decode::decode(&r.scb);
        out.push(StreamPackage {
            seq: r.seq,
            event_type: r.event_type,
            hash: r.hash,
            prev_hash: r.prev_hash,
            value: value?,
        });
    }

    Ok(out)
}
