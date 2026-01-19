use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::adapter::append::{AppendError, append_package};

pub async fn append_event(
    db: &DatabaseConnection,
    stream_id: Uuid,
    event_type: &str,
    payload: &strata::value::Value,
) -> Result<(), AppendError> {
    append_package(db, stream_id, event_type, payload).await?;
    Ok(())
}
