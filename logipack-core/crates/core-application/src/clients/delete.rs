use core_data::repository::clients_repo::{self, ClientError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum DeleteClientError {
    #[error("forbidden")]
    Forbidden,
    #[error("{0}")]
    DeleteClientError(#[from] ClientError),
}

pub async fn delete_client(
    db: &DatabaseConnection,
    actor: &ActorContext,
    id: Uuid,
) -> Result<Uuid, DeleteClientError> {
    // Only admin can create clients
    if !actor.is_admin() {
        return Err(DeleteClientError::Forbidden);
    }

    clients_repo::ClientsRepo::delete_client(db, id).await?;

    Ok(id)
}
