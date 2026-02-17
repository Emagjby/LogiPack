use core_data::repository::users_repo::{UserError, UserRepo};
use sea_orm::DatabaseConnection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MeError {
    #[error("user not found")]
    NotFound,
    #[error("db error: {0}")]
    DbError(#[from] UserError),
}

/// Returns the role string for the user identified by `auth0_sub`.
///
/// - If the user does not exist in the database, returns `MeError::NotFound`.
/// - If the user exists but has no role assigned, returns `""`.
pub async fn get_me_role(db: &DatabaseConnection, auth0_sub: &str) -> Result<String, MeError> {
    let role = UserRepo::get_role_by_auth0_sub(db, auth0_sub)
        .await?
        .ok_or(MeError::NotFound)?;

    Ok(role)
}
