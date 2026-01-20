use sea_orm::ActiveValue::{self, Set};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use thiserror::Error;
use uuid::Uuid;

use crate::entity::{shipment_status_history, shipments};
use core_domain::shipment::ShipmentStatus;

#[derive(Debug, Error)]
pub enum ShipmentSnapshotError {
    #[error("db error: {0}")]
    DbError(#[from] DbErr),
}

pub struct ShipmentsRepo;

impl ShipmentsRepo {
    /// Insert initial snapshot of shipment creation
    pub async fn insert_snapshot(
        db: &DatabaseConnection,
        shipment_id: Uuid,
        client_id: Uuid,
        status: ShipmentStatus,
        office_id: Option<Uuid>,
    ) -> Result<(), ShipmentSnapshotError> {
        let model = shipments::ActiveModel {
            id: Set(shipment_id),
            client_id: Set(client_id),
            current_status: Set(status.to_string()),
            current_office_id: Set(office_id),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };

        model.insert(db).await?;
        Ok(())
    }

    /// Insert history row for any status change
    pub async fn insert_history(
        db: &DatabaseConnection,
        shipment_id: Uuid,
        from_status: Option<ShipmentStatus>,
        to_status: ShipmentStatus,
        actor_user_id: Option<Uuid>,
        office_id: Option<Uuid>,
        notes: Option<String>,
    ) -> Result<(), ShipmentSnapshotError> {
        let model = shipment_status_history::ActiveModel {
            id: ActiveValue::NotSet,
            shipment_id: Set(shipment_id),
            from_status: Set(from_status.map(|s| s.to_string())),
            to_status: Set(to_status.to_string()),
            actor_user_id: Set(actor_user_id),
            office_id: Set(office_id),
            notes: Set(notes),
            changed_at: Set(chrono::Utc::now().into()),
        };

        model.insert(db).await?;
        Ok(())
    }

    /// Update snapshot on transition
    pub async fn update_snapshot_status(
        db: &DatabaseConnection,
        shipment_id: Uuid,
        new_status: ShipmentStatus,
        new_office_id: Option<Uuid>,
    ) -> Result<(), ShipmentSnapshotError> {
        let mut model: shipments::ActiveModel = shipments::Entity::find_by_id(shipment_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("shipment not found".into()))?
            .into();

        model.current_status = Set(new_status.to_string());

        if new_office_id.is_some() {
            model.current_office_id = Set(new_office_id);
        }

        model.updated_at = Set(chrono::Utc::now().into());

        model.update(db).await?;
        Ok(())
    }

    /// Read snapshot
    pub async fn get_snapshot(
        db: &DatabaseConnection,
        shipment_id: Uuid,
    ) -> Result<shipments::Model, ShipmentSnapshotError> {
        Ok(shipments::Entity::find_by_id(shipment_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("shipment not found".into()))?)
    }
}
