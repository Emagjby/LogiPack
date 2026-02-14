use core_data::repository::employees_repo::{self, EmployeeError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;
use crate::validation::employee::{EmployeeValidationError, validate_full_name};

#[derive(Debug, Clone)]
pub struct UpdateEmployee {
    pub id: Uuid,
    pub full_name: Option<String>,
}

#[derive(Debug, Error)]
pub enum UpdateEmployeeError {
    #[error("forbidden")]
    Forbidden,
    #[error("validation error: {0}")]
    Validation(#[from] EmployeeValidationError),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    EmployeeError(EmployeeError),
}

pub async fn update_employee(
    db: &DatabaseConnection,
    actor: &ActorContext,
    input: UpdateEmployee,
) -> Result<Uuid, UpdateEmployeeError> {
    // Only admin can update employees
    if !actor.is_admin() {
        return Err(UpdateEmployeeError::Forbidden);
    }

    if let Some(ref full_name) = input.full_name {
        validate_full_name(full_name)?;
    }

    employees_repo::EmployeesRepo::update_employee(db, input.id, input.full_name)
        .await
        .map_err(|e| match e {
            EmployeeError::RecordNotFound => UpdateEmployeeError::NotFound,
            other => UpdateEmployeeError::EmployeeError(other),
        })?;

    Ok(input.id)
}
