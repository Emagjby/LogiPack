use core_data::repository::employees_repo::{self, EmployeeError};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::actor::ActorContext;
use crate::validation::employee::{EmployeeValidationError, validate_full_name};

#[derive(Debug, Clone)]
pub struct CreateEmployee {
    pub user_id: Uuid,
    pub full_name: String,
}

#[derive(Debug, Error)]
pub enum CreateEmployeeError {
    #[error("forbidden")]
    Forbidden,
    #[error("validation error: {0}")]
    Validation(#[from] EmployeeValidationError),
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

    validate_full_name(&input.full_name)?;

    let employee_id = Uuid::new_v4();

    employees_repo::EmployeesRepo::create_employee(db, employee_id, input.user_id, input.full_name)
        .await?;

    Ok(employee_id)
}
