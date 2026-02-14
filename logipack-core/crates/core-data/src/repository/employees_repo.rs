use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter,
};
use thiserror::Error;
use uuid::Uuid;

use crate::entity::employees;

#[derive(Debug, Error)]
pub enum EmployeeError {
    #[error("db error: {0}")]
    EmployeeDbError(#[from] DbErr),
    #[error("employee not found")]
    RecordNotFound,
}

pub struct EmployeesRepo;

impl EmployeesRepo {
    /// Creates a new employee
    pub async fn create_employee(
        db: &DatabaseConnection,
        id: Uuid,
        user_id: Uuid,
        full_name: String,
    ) -> Result<(), EmployeeError> {
        let model = employees::ActiveModel {
            id: Set(id),
            user_id: Set(user_id),
            full_name: Set(full_name),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            deleted_at: Set(None),
        };

        model.insert(db).await?;
        Ok(())
    }

    /// Gets employee by id
    pub async fn get_employee_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<employees::Model>, EmployeeError> {
        let retrieved = employees::Entity::find_by_id(id).one(db).await?;

        if retrieved.is_none() || retrieved.as_ref().unwrap().deleted_at.is_some() {
            return Err(EmployeeError::RecordNotFound);
        }

        Ok(retrieved)
    }

    /// Lists all employees
    pub async fn list_employees(
        db: &DatabaseConnection,
    ) -> Result<Vec<employees::Model>, EmployeeError> {
        let retrieved = employees::Entity::find()
            .filter(employees::Column::DeletedAt.is_null())
            .all(db)
            .await?;
        Ok(retrieved)
    }

    /// Updates an employee's information
    pub async fn update_employee(
        db: &DatabaseConnection,
        id: Uuid,
        full_name: Option<String>,
    ) -> Result<(), EmployeeError> {
        let mut model = employees::Entity::find_by_id(id)
            .filter(employees::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(EmployeeError::RecordNotFound)?
            .into_active_model();

        if let Some(full_name) = full_name {
            model.full_name = Set(full_name);
        }

        model.updated_at = Set(chrono::Utc::now().into());

        model.update(db).await?;
        Ok(())
    }

    /// Soft deletes an employee by id
    pub async fn delete_employee(db: &DatabaseConnection, id: Uuid) -> Result<(), EmployeeError> {
        let result = employees::Entity::update_many()
            .col_expr(
                employees::Column::DeletedAt,
                sea_orm::sea_query::Expr::cust("NOW()"),
            )
            .filter(employees::Column::Id.eq(id))
            .filter(employees::Column::DeletedAt.is_null())
            .exec(db)
            .await?;

        if result.rows_affected == 0 {
            return Err(EmployeeError::RecordNotFound);
        }

        Ok(())
    }
}
