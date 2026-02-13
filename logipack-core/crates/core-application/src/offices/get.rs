use core_data::{
    entity::offices,
    repository::offices_repo::{self, OfficeError},
};
use sea_orm::DatabaseConnection;
use thiserror::Error;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum GetOfficeError {
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    OfficeError(#[from] OfficeError),
}

pub async fn get_office(
    db: &DatabaseConnection,
    actor: &ActorContext,
    id: uuid::Uuid,
) -> Result<Option<offices::Model>, GetOfficeError> {
    // Only admin can get offices
    if !actor.is_admin() {
        return Err(GetOfficeError::Forbidden);
    }

    let result = offices_repo::OfficesRepo::get_office_by_id(db, id)
        .await
        .map_err(|e| match e {
            OfficeError::RecordNotFound => GetOfficeError::NotFound,
            other => GetOfficeError::OfficeError(other),
        })?;

    Ok(result)
}
