use core_data::{
    entity::clients,
    repository::clients_repo::{self, ClientError},
};
use sea_orm::DatabaseConnection;
use thiserror::Error;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum GetClientError {
    #[error("forbidden")]
    Forbidden,
    #[error("{0}")]
    ClientError(#[from] ClientError),
}

pub async fn get_client(
    db: &DatabaseConnection,
    actor: &ActorContext,
    id: uuid::Uuid,
) -> Result<Option<clients::Model>, GetClientError> {
    // Only admin can create clients
    if !actor.is_admin() {
        return Err(GetClientError::Forbidden);
    }

    let result = clients_repo::ClientsRepo::get_client_by_id(db, id).await?;

    Ok(result)
}
