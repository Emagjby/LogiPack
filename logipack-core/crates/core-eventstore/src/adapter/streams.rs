use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use thiserror::Error;
use uuid::Uuid;

use crate::schema::streams;

#[derive(Debug, Error)]
pub enum EnsureStreamError {
    #[error("db error: {0}")]
    Db(#[from] DbErr),
}

pub async fn ensure_stream(
    db: &DatabaseConnection,
    stream_id: Uuid,
    kind: &str,
) -> Result<(), EnsureStreamError> {
    let existing = streams::Entity::find_by_id(stream_id).one(db).await?;
    if existing.is_some() {
        return Ok(());
    }

    let model = streams::ActiveModel {
        id: Set(stream_id),
        kind: Set(kind.to_string()),
        head_hash: Set(None),
        created_at: sea_orm::ActiveValue::NotSet,
    };

    model.insert(db).await?;
    Ok(())
}
