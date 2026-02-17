use core_data::repository::employees_repo::{self, EmployeeError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;

#[derive(Debug, Clone)]
pub struct UpdateEmployee {
    pub id: Uuid,
}

#[derive(Debug, Error)]
pub enum UpdateEmployeeError {
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    EmployeeError(EmployeeError),
}

/// Updates an employee record.
///
/// Currently this only bumps `updated_at` (timestamp touch). No user-visible
/// mutable fields are exposed yet — extend `UpdateEmployee` with fields such as
/// role, name, etc. when the product requires it.
pub async fn update_employee(
    db: &DatabaseConnection,
    actor: &ActorContext,
    input: UpdateEmployee,
) -> Result<Uuid, UpdateEmployeeError> {
    // Only admin can update employees
    if !actor.is_admin() {
        return Err(UpdateEmployeeError::Forbidden);
    }

    employees_repo::EmployeesRepo::update_employee(db, input.id)
        .await
        .map_err(|e| match e {
            EmployeeError::RecordNotFound => UpdateEmployeeError::NotFound,
            other => UpdateEmployeeError::EmployeeError(other),
        })?;

    Ok(input.id)
}
