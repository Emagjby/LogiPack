use core_data::repository::offices_repo::{self, OfficeError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum DeleteOfficeError {
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    DeleteOfficeError(#[from] OfficeError),
}

pub async fn delete_office(
    db: &DatabaseConnection,
    actor: &ActorContext,
    id: Uuid,
) -> Result<Uuid, DeleteOfficeError> {
    // Only admin can delete offices
    if !actor.is_admin() {
        return Err(DeleteOfficeError::Forbidden);
    }

    offices_repo::OfficesRepo::delete_office(db, id)
        .await
        .map_err(|e| match e {
            OfficeError::RecordNotFound => DeleteOfficeError::NotFound,
            other => DeleteOfficeError::DeleteOfficeError(other),
        })?;

    Ok(id)
}
