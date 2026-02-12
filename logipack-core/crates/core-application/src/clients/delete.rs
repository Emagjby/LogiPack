use core_data::repository::clients_repo::{self, ClientError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum DeleteClientError {
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    DeleteClientError(#[from] ClientError),
}

pub async fn delete_client(
    db: &DatabaseConnection,
    actor: &ActorContext,
    id: Uuid,
) -> Result<Uuid, DeleteClientError> {
    // Only admin can delete clients
    if !actor.is_admin() {
        return Err(DeleteClientError::Forbidden);
    }

    clients_repo::ClientsRepo::delete_client(db, id)
        .await
        .map_err(|e| match e {
            ClientError::RecordNotFound => DeleteClientError::NotFound,
            other => DeleteClientError::DeleteClientError(other),
        })?;

    Ok(id)
}
