use core_data::repository::clients_repo::{self, ClientError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;
use crate::validation::client::{
    ClientValidationError, validate_email, validate_name, validate_phone,
};

#[derive(Debug, Clone)]
pub struct UpdateClient {
    pub id: Uuid,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Error)]
pub enum UpdateClientError {
    #[error("forbidden")]
    Forbidden,
    #[error("validation error: {0}")]
    Validation(#[from] ClientValidationError),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    UpdateClientError(#[from] ClientError),
}

pub async fn update_client(
    db: &DatabaseConnection,
    actor: &ActorContext,
    input: UpdateClient,
) -> Result<Uuid, UpdateClientError> {
    // Only admin can update clients
    if !actor.is_admin() {
        return Err(UpdateClientError::Forbidden);
    }

    if let Some(ref name) = input.name {
        validate_name(name)?;
    }

    if let Some(ref email) = input.email {
        validate_email(email)?;
    }

    validate_phone(input.phone.as_deref())?;

    clients_repo::ClientsRepo::update_client(db, input.id, input.name, input.phone, input.email)
        .await
        .map_err(|e| match e {
            ClientError::RecordNotFound => UpdateClientError::NotFound,
            other => UpdateClientError::UpdateClientError(other),
        })?;

    Ok(input.id)
}
