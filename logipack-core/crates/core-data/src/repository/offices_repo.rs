use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter,
};
use thiserror::Error;
use uuid::Uuid;

use crate::entity::offices;

#[derive(Debug, Error)]
pub enum OfficeError {
    #[error("db error: {0}")]
    OfficeDbError(#[from] DbErr),
    #[error("office not found")]
    RecordNotFound,
}

pub struct OfficesRepo;

impl OfficesRepo {
    /// Creates a new office
    pub async fn create_office(
        db: &DatabaseConnection,
        id: Uuid,
        name: String,
        city: String,
        address: String,
    ) -> Result<(), OfficeError> {
        let model = offices::ActiveModel {
            id: Set(id),
            name: Set(name),
            city: Set(city),
            address: Set(address),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            deleted_at: Set(None),
        };

        model.insert(db).await?;
        Ok(())
    }

    /// Gets office by id
    pub async fn get_office_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<offices::Model>, OfficeError> {
        let retrieved = offices::Entity::find_by_id(id).one(db).await?;

        // If the office is not found or is soft-deleted, return RecordNotFound error
        if retrieved.is_none() || retrieved.as_ref().unwrap().deleted_at.is_some() {
            return Err(OfficeError::RecordNotFound);
        }

        Ok(retrieved)
    }

    /// Lists all offices
    pub async fn list_offices(db: &DatabaseConnection) -> Result<Vec<offices::Model>, OfficeError> {
        // Only return offices that are not soft-deleted
        let retrieved = offices::Entity::find()
            .filter(offices::Column::DeletedAt.is_null())
            .all(db)
            .await?;
        Ok(retrieved)
    }

    /// Updates an office's information
    pub async fn update_office(
        db: &DatabaseConnection,
        id: Uuid,
        name: Option<String>,
        city: Option<String>,
        address: Option<String>,
    ) -> Result<(), OfficeError> {
        // Exlude soft-deleted records from the search
        let mut model = offices::Entity::find_by_id(id)
            .filter(offices::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(OfficeError::RecordNotFound)?
            .into_active_model();

        if let Some(name) = name {
            model.name = Set(name);
        }

        if let Some(city) = city {
            model.city = Set(city);
        }

        if let Some(address) = address {
            model.address = Set(address);
        }

        model.updated_at = Set(chrono::Utc::now().into());

        model.update(db).await?;
        Ok(())
    }

    /// Soft deletes an office by id
    pub async fn delete_office(db: &DatabaseConnection, id: Uuid) -> Result<(), OfficeError> {
        let result = offices::Entity::update_many()
            .col_expr(
                offices::Column::DeletedAt,
                sea_orm::sea_query::Expr::cust("NOW()"),
            )
            .filter(offices::Column::Id.eq(id))
            .filter(offices::Column::DeletedAt.is_null())
            .exec(db)
            .await?;

        if result.rows_affected == 0 {
            return Err(OfficeError::RecordNotFound);
        }

        Ok(())
    }
}
