use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter,
};
use thiserror::Error;
use uuid::Uuid;

use crate::entity::clients;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("db error: {0}")]
    ClientDbError(#[from] DbErr),
    #[error("client not found")]
    RecordNotFound,
}

pub struct ClientsRepo;

impl ClientsRepo {
    /// Creates a new client
    pub async fn create_client(
        db: &DatabaseConnection,
        id: Uuid,
        name: String,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<(), ClientError> {
        let model = clients::ActiveModel {
            id: Set(id),
            name: Set(name),
            phone: Set(phone),
            email: Set(email),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            deleted_at: Set(None),
        };

        model.insert(db).await?;
        Ok(())
    }

    /// Gets client by id
    pub async fn get_client_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<clients::Model>, ClientError> {
        let retrieved = clients::Entity::find_by_id(id).one(db).await?;

        if retrieved.is_none() || retrieved.as_ref().unwrap().deleted_at.is_some() {
            return Err(ClientError::RecordNotFound);
        }

        Ok(retrieved)
    }

    /// Lists all clients
    pub async fn list_clients(db: &DatabaseConnection) -> Result<Vec<clients::Model>, ClientError> {
        let retrieved = clients::Entity::find()
            .filter(clients::Column::DeletedAt.is_null())
            .all(db)
            .await?;
        Ok(retrieved)
    }

    /// Soft deletes a client by id
    pub async fn delete_client(db: &DatabaseConnection, id: Uuid) -> Result<(), ClientError> {
        let result = clients::Entity::update_many()
            .col_expr(
                clients::Column::DeletedAt,
                sea_orm::sea_query::Expr::cust("NOW()"),
            )
            .filter(clients::Column::Id.eq(id))
            .filter(clients::Column::DeletedAt.is_null())
            .exec(db)
            .await?;

        if result.rows_affected == 0 {
            return Err(ClientError::RecordNotFound);
        }

        Ok(())
    }

    /// Updates a client's information
    pub async fn update_client(
        db: &DatabaseConnection,
        id: Uuid,
        name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<(), ClientError> {
        // Exclude soft-deleted records from the search
        let mut model = clients::Entity::find_by_id(id)
            .filter(clients::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(ClientError::RecordNotFound)?
            .into_active_model();

        if let Some(name) = name {
            model.name = Set(name);
        }

        if let Some(phone) = phone {
            model.phone = Set(Some(phone));
        }

        if let Some(email) = email {
            model.email = Set(Some(email));
        }

        model.updated_at = Set(chrono::Utc::now().into());

        model.update(db).await?;
        Ok(())
    }
}
