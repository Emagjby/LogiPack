use core_data::{
    entity::shipments,
    repository::shipments_repo::{ShipmentSnapshotError, ShipmentsRepo},
};
use sea_orm::DatabaseConnection;

pub async fn list_shipments(
    db: &DatabaseConnection,
) -> Result<Vec<shipments::Model>, ShipmentSnapshotError> {
    ShipmentsRepo::list_snapshots(db).await
}
