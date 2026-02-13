use core_data::{
    entity::offices,
    repository::offices_repo::{self, OfficeError},
};
use sea_orm::DatabaseConnection;
use thiserror::Error;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum ListOfficesError {
    #[error("forbidden")]
    Forbidden,
    #[error("{0}")]
    OfficeError(#[from] OfficeError),
}

pub async fn list_offices(
    db: &DatabaseConnection,
    actor: &ActorContext,
) -> Result<Vec<offices::Model>, ListOfficesError> {
    // Only admin can list offices
    if !actor.is_admin() {
        return Err(ListOfficesError::Forbidden);
    }

    let result = offices_repo::OfficesRepo::list_offices(db).await?;

    Ok(result)
}
