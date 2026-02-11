use core_data::{
    entity::clients,
    repository::clients_repo::{self, ClientError},
};
use sea_orm::DatabaseConnection;
use thiserror::Error;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum ListClientsError {
    #[error("forbidden")]
    Forbidden,
    #[error("{0}")]
    ClientError(#[from] ClientError),
}

pub async fn list_clients(
    db: &DatabaseConnection,
    actor: &ActorContext,
) -> Result<Vec<clients::Model>, ListClientsError> {
    // Only admin can create clients
    if !actor.is_admin() {
        return Err(ListClientsError::Forbidden);
    }

    let result = clients_repo::ClientsRepo::list_clients(db).await?;

    Ok(result)
}
