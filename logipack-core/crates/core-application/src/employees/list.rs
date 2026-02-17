use core_data::repository::employees_repo::{self, EmployeeError, EmployeeWithUser};
use sea_orm::DatabaseConnection;
use thiserror::Error;

use crate::actor::ActorContext;

#[derive(Debug, Error)]
pub enum ListEmployeesError {
    #[error("forbidden")]
    Forbidden,
    #[error("{0}")]
    EmployeeError(#[from] EmployeeError),
}

pub async fn list_employees(
    db: &DatabaseConnection,
    actor: &ActorContext,
) -> Result<Vec<EmployeeWithUser>, ListEmployeesError> {
    // Only admin can list employees
    if !actor.is_admin() {
        return Err(ListEmployeesError::Forbidden);
    }

    let result = employees_repo::EmployeesRepo::list_employees(db).await?;

    Ok(result)
}
