use core_data::repository::clients_repo::{self, ClientError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;
use crate::validation::client::{ClientValidationError, validate_client};

#[derive(Debug, Clone)]
pub struct CreateClient {
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Error)]
pub enum CreateClientError {
    #[error("forbidden")]
    Forbidden,
    #[error("validation error: {0}")]
    Validation(#[from] ClientValidationError),
    #[error("{0}")]
    ClientCreationError(#[from] ClientError),
}

pub async fn create_client(
    db: &DatabaseConnection,
    actor: &ActorContext,
    input: CreateClient,
) -> Result<Uuid, CreateClientError> {
    // Only admin can create clients
    if !actor.is_admin() {
        return Err(CreateClientError::Forbidden);
    }

    let email = input.email.as_deref().ok_or(CreateClientError::Validation(
        ClientValidationError::InvalidEmail,
    ))?;

    validate_client(&input.name, email, input.phone.as_deref())?;

    let client_id = Uuid::new_v4();

    clients_repo::ClientsRepo::create_client(db, client_id, input.name, input.phone, input.email)
        .await?;

    Ok(client_id)
}
