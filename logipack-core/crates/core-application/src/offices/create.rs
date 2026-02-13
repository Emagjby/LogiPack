use core_data::repository::offices_repo::{self, OfficeError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;
use crate::validation::office::{OfficeValidationError, validate_office};

#[derive(Debug, Clone)]
pub struct CreateOffice {
    pub name: String,
    pub city: String,
    pub address: String,
}

#[derive(Debug, Error)]
pub enum CreateOfficeError {
    #[error("forbidden")]
    Forbidden,
    #[error("validation error: {0}")]
    Validation(#[from] OfficeValidationError),
    #[error("{0}")]
    OfficeCreationError(#[from] OfficeError),
}

pub async fn create_office(
    db: &DatabaseConnection,
    actor: &ActorContext,
    input: CreateOffice,
) -> Result<Uuid, CreateOfficeError> {
    // Only admin can create offices
    if !actor.is_admin() {
        return Err(CreateOfficeError::Forbidden);
    }

    validate_office(&input.name, &input.city, &input.address)?;

    let office_id = Uuid::new_v4();

    offices_repo::OfficesRepo::create_office(db, office_id, input.name, input.city, input.address)
        .await?;

    Ok(office_id)
}
