use core_data::{
    entity::shipments,
    repository::shipments_repo::{ShipmentSnapshotError, ShipmentsRepo},
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_shipment(
    db: &DatabaseConnection,
    shipment_id: Uuid,
) -> Result<shipments::Model, ShipmentSnapshotError> {
    ShipmentsRepo::get_snapshot(db, shipment_id).await
}
