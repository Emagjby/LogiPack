use core_data::repository::employees_repo::{self, EmployeeError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum DeleteEmployeeError {
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    EmployeeError(EmployeeError),
}

pub async fn delete_employee(
    db: &DatabaseConnection,
    actor: &ActorContext,
    id: Uuid,
) -> Result<Uuid, DeleteEmployeeError> {
    // Only admin can delete employees
    if !actor.is_admin() {
        return Err(DeleteEmployeeError::Forbidden);
    }

    employees_repo::EmployeesRepo::delete_employee(db, id)
        .await
        .map_err(|e| match e {
            EmployeeError::RecordNotFound => DeleteEmployeeError::NotFound,
            other => DeleteEmployeeError::EmployeeError(other),
        })?;

    Ok(id)
}
