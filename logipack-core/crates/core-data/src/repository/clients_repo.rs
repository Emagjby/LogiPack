use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
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

        if retrieved.is_none() {
            return Err(ClientError::RecordNotFound);
        }

        Ok(retrieved)
    }

    /// Lists all clients
    pub async fn list_clients(db: &DatabaseConnection) -> Result<Vec<clients::Model>, ClientError> {
        let retrieved = clients::Entity::find().all(db).await?;
        Ok(retrieved)
    }

    /// Deletes a client by id
    pub async fn delete_client(db: &DatabaseConnection, id: Uuid) -> Result<(), ClientError> {
        let result = clients::Entity::delete_by_id(id).exec(db).await?;

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
        let mut model: clients::ActiveModel = clients::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(ClientError::RecordNotFound)?
            .into();

        if let Some(name) = name {
            model.name = Set(name);
        }

        if let Some(phone) = phone {
            model.phone = Set(Some(phone));
        }

        if let Some(email) = email {
            model.email = Set(Some(email));
        }

        model.update(db).await?;
        Ok(())
    }
}
