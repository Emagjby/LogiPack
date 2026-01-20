use sea_orm::{
    ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, QueryFilter,
    QuerySelect, TransactionTrait,
};
use strata::{list, string};
use thiserror::Error;
use uuid::Uuid;

use crate::hashing::{HashedPackage, hash_strata_value};
use crate::schema::{packages, streams};

#[derive(Debug, Error)]
pub enum AppendError {
    #[error("stream not found")]
    StreamNotFound,
    #[error("encoding error: {0:?}")]
    Encode(strata::error::EncodeError),
    #[error("db error: {0}")]
    Db(sea_orm::DbErr),
}

impl From<DbErr> for AppendError {
    fn from(err: DbErr) -> Self {
        AppendError::Db(err)
    }
}

impl From<strata::error::EncodeError> for AppendError {
    fn from(err: strata::error::EncodeError) -> Self {
        AppendError::Encode(err)
    }
}

/// Appends a Strata value as a package to an existing stream.
///
/// Guarantees:
/// - append is atomic
/// - seq is strictly monotonic per stream
/// - prev_hash links correctly
/// - streams.head_hash is updated
pub async fn append_package(
    db: &DatabaseConnection,
    stream_id: Uuid,
    event_type: &str,
    value: &strata::value::Value,
) -> Result<HashedPackage, AppendError> {
    let scoped = list![string!(stream_id.to_string()), value.clone()];

    let hashed = hash_strata_value(&scoped)?;

    let txn = db.begin().await?;

    let result = append_package_txn(&txn, stream_id, event_type, &hashed).await;

    match result {
        Ok(_) => {
            txn.commit().await?;
            Ok(hashed)
        }
        Err(err) => {
            txn.rollback().await.ok();
            Err(err)
        }
    }
}

async fn append_package_txn(
    txn: &DatabaseTransaction,
    stream_id: Uuid,
    event_type: &str,
    hashed: &HashedPackage,
) -> Result<(), AppendError> {
    // Fetch stream to ensure it exists and get current head_hash.
    let stream = streams::Entity::find_by_id(stream_id)
        .lock_exclusive()
        .one(txn)
        .await?
        .ok_or(AppendError::StreamNotFound)?;

    // Determine prev_hash and seq for the new package.
    let prev_hash = stream.head_hash.clone();
    let last_seq = packages::Entity::find()
        .filter(packages::Column::StreamId.eq(stream_id))
        .select_only()
        .column_as(packages::Column::Seq.max(), "max_seq")
        .into_tuple()
        .one(txn)
        .await?
        .flatten();

    let next_seq = last_seq.unwrap_or(0) + 1;

    // Insert the new package.
    let pkg = packages::ActiveModel {
        hash: sea_orm::ActiveValue::Set(hashed.hash.clone()),
        stream_id: sea_orm::ActiveValue::Set(stream_id),
        prev_hash: sea_orm::ActiveValue::Set(prev_hash.clone()),
        seq: sea_orm::ActiveValue::Set(next_seq),
        event_type: sea_orm::ActiveValue::Set(event_type.to_owned()),
        scb: sea_orm::ActiveValue::Set(hashed.scb.clone()),
        created_at: sea_orm::ActiveValue::NotSet, //Db def
    };

    packages::Entity::insert(pkg).exec(txn).await?;

    // Update the stream head_hash.
    let mut stream_update: streams::ActiveModel = stream.into();
    stream_update.head_hash = sea_orm::ActiveValue::Set(Some(hashed.hash.clone()));
    streams::Entity::update(stream_update).exec(txn).await?;

    Ok(())
}
