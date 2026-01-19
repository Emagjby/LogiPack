use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::adapter::append::{AppendError, append_package};

pub async fn append_shipment_created(
    db: &DatabaseConnection,
    shipment_id: Uuid,
    event_type: &str,
) -> Result<(), AppendError> {
    let payload = strata::value::Value::Map(Default::default());
    append_package(db, shipment_id, event_type, &payload).await?;
    Ok(())
}
