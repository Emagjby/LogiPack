use core_eventstore::adapter::read::{ReadError, StreamPackage, read_stream_packages};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum TimelineError {
    #[error("eventstore read error: {0:?}")]
    Read(#[from] ReadError),
}

pub async fn read_timeline(
    db: &DatabaseConnection,
    shipment_id: Uuid,
) -> Result<Vec<StreamPackage>, TimelineError> {
    let items = read_stream_packages(db, shipment_id).await?;
    Ok(items)
}
