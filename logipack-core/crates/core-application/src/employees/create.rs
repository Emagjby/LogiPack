use core_data::repository::employees_repo::{self, EmployeeError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;

#[derive(Debug, Clone)]
pub struct CreateEmployee {
    pub user_id: Uuid,
}

#[derive(Debug, Error)]
pub enum CreateEmployeeError {
    #[error("forbidden")]
    Forbidden,
    #[error("{0}")]
    EmployeeCreationError(#[from] EmployeeError),
}

pub async fn create_employee(
    db: &DatabaseConnection,
    actor: &ActorContext,
    input: CreateEmployee,
) -> Result<Uuid, CreateEmployeeError> {
    // Only admin can create employees
    if !actor.is_admin() {
        return Err(CreateEmployeeError::Forbidden);
    }

    let employee_id = Uuid::new_v4();

    employees_repo::EmployeesRepo::create_employee(db, employee_id, input.user_id).await?;

    Ok(employee_id)
}
