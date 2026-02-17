use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter,
};
use thiserror::Error;
use uuid::Uuid;

use crate::entity::{employees, users};

#[derive(Debug, Error)]
pub enum EmployeeError {
    #[error("db error: {0}")]
    EmployeeDbError(#[from] DbErr),
    #[error("employee not found")]
    RecordNotFound,
    #[error("related user not found for employee")]
    RelatedUserNotFound,
}

#[derive(Debug, Clone)]
pub struct EmployeeWithUser {
    pub employee: employees::Model,
    pub user: users::Model,
}

pub struct EmployeesRepo;

impl EmployeesRepo {
    /// Creates a new employee
    pub async fn create_employee(
        db: &DatabaseConnection,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<(), EmployeeError> {
        let model = employees::ActiveModel {
            id: Set(id),
            user_id: Set(user_id),
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
    ) -> Result<EmployeeWithUser, EmployeeError> {
        let retrieved = employees::Entity::find_by_id(id)
            .filter(employees::Column::DeletedAt.is_null())
            .find_also_related(users::Entity)
            .one(db)
            .await?;

        let (employee, user) = match retrieved {
            Some((employee, Some(user))) => (employee, user),
            _ => return Err(EmployeeError::RecordNotFound),
        };

        Ok(EmployeeWithUser { employee, user })
    }

    /// Lists all employees
    pub async fn list_employees(
        db: &DatabaseConnection,
    ) -> Result<Vec<EmployeeWithUser>, EmployeeError> {
        let retrieved = employees::Entity::find()
            .filter(employees::Column::DeletedAt.is_null())
            .find_also_related(users::Entity)
            .all(db)
            .await?;

        let mut employees_with_users = Vec::with_capacity(retrieved.len());
        for (employee, user) in retrieved {
            let user = user.ok_or(EmployeeError::RelatedUserNotFound)?;
            employees_with_users.push(EmployeeWithUser { employee, user });
        }

        Ok(employees_with_users)
    }

    /// Updates an employee's `updated_at` timestamp (touch).
    ///
    /// Currently no user-visible fields are modified — this acts as a
    /// timestamp bump only. Extend when mutable employee fields are added.
    pub async fn update_employee(db: &DatabaseConnection, id: Uuid) -> Result<(), EmployeeError> {
        let mut model = employees::Entity::find_by_id(id)
            .filter(employees::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(EmployeeError::RecordNotFound)?
            .into_active_model();

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
