use core_data::repository::offices_repo::{self, OfficeError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;
use crate::validation::office::{
    OfficeValidationError, validate_address, validate_city, validate_name,
};

#[derive(Debug, Clone)]
pub struct UpdateOffice {
    pub id: Uuid,
    pub name: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Error)]
pub enum UpdateOfficeError {
    #[error("forbidden")]
    Forbidden,
    #[error("validation error: {0}")]
    Validation(#[from] OfficeValidationError),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    UpdateOfficeError(#[from] OfficeError),
}

pub async fn update_office(
    db: &DatabaseConnection,
    actor: &ActorContext,
    input: UpdateOffice,
) -> Result<Uuid, UpdateOfficeError> {
    // Only admin can update offices
    if !actor.is_admin() {
        return Err(UpdateOfficeError::Forbidden);
    }

    if let Some(ref name) = input.name {
        validate_name(name)?;
    }

    if let Some(ref city) = input.city {
        validate_city(city)?;
    }

    if let Some(ref address) = input.address {
        validate_address(address)?;
    }

    offices_repo::OfficesRepo::update_office(db, input.id, input.name, input.city, input.address)
        .await
        .map_err(|e| match e {
            OfficeError::RecordNotFound => UpdateOfficeError::NotFound,
            other => UpdateOfficeError::UpdateOfficeError(other),
        })?;

    Ok(input.id)
}
