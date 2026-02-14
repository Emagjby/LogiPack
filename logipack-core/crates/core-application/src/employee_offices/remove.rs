use core_data::repository::employee_offices_repo::{EmployeeOfficeError, EmployeeOfficesRepo};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;

#[derive(Debug, Clone)]
pub struct RemoveOffice {
    pub employee_id: Uuid,
    pub office_id: Uuid,
}

#[derive(Debug, Error)]
pub enum RemoveOfficeError {
    #[error("forbidden")]
    Forbidden,
    #[error("employee not found")]
    EmployeeNotFound,
    #[error("office not found")]
    OfficeNotFound,
    #[error("{0}")]
    RemoveError(#[from] EmployeeOfficeError),
}

pub async fn remove_office(
    db: &DatabaseConnection,
    actor: &ActorContext,
    input: RemoveOffice,
) -> Result<(), RemoveOfficeError> {
    if !actor.is_admin() {
        return Err(RemoveOfficeError::Forbidden);
    }

    EmployeeOfficesRepo::remove_office(db, input.employee_id, input.office_id)
        .await
        .map_err(|e| match e {
            EmployeeOfficeError::EmployeeNotFound => RemoveOfficeError::EmployeeNotFound,
            EmployeeOfficeError::OfficeNotFound => RemoveOfficeError::OfficeNotFound,
            other => RemoveOfficeError::RemoveError(other),
        })?;

    Ok(())
}
