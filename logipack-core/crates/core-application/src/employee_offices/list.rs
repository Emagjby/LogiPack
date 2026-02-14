use core_data::repository::employee_offices_repo::{EmployeeOfficeError, EmployeeOfficesRepo};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum ListEmployeeOfficesError {
    #[error("forbidden")]
    Forbidden,
    #[error("employee not found")]
    EmployeeNotFound,
    #[error("{0}")]
    ListError(#[from] EmployeeOfficeError),
}

pub async fn list_employee_offices(
    db: &DatabaseConnection,
    actor: &ActorContext,
    employee_id: Uuid,
) -> Result<Vec<Uuid>, ListEmployeeOfficesError> {
    if !actor.is_admin() {
        return Err(ListEmployeeOfficesError::Forbidden);
    }

    let office_ids = EmployeeOfficesRepo::list_offices(db, employee_id)
        .await
        .map_err(|e| match e {
            EmployeeOfficeError::EmployeeNotFound => ListEmployeeOfficesError::EmployeeNotFound,
            other => ListEmployeeOfficesError::ListError(other),
        })?;

    Ok(office_ids)
}
