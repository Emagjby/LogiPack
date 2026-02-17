use core_data::repository::users_repo::{UserError, UserRepo};
use sea_orm::DatabaseConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::validation::user::{UserValidationError, validate_email, validate_name};

#[derive(Debug, Clone)]
pub struct EnsureUser {
    pub auth0_sub: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Error)]
pub enum EnsureUserError {
    #[error("validation error: {0}")]
    Validation(#[from] UserValidationError),
    #[error("db error: {0}")]
    DbError(String),
    #[error("email already linked to another account")]
    EmailAlreadyLinked,
    #[error("user not found")]
    UserNotFound,
    #[error("invalid auth0 subject identifier")]
    InvalidAuth0Sub,
}

impl From<UserError> for EnsureUserError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::EmailAlreadyLinked => EnsureUserError::EmailAlreadyLinked,
            UserError::RecordNotFound => EnsureUserError::UserNotFound,
            other => EnsureUserError::DbError(other.to_string()),
        }
    }
}

/// Ensures a user row exists for the given Auth0 subject.
///
/// - Validates `name` and `email`.
/// - If a user with the given `auth0_sub` exists, updates `name`/`email` if changed.
/// - If missing, creates a new user with no roles (default policy).
/// - Never modifies roles.
///
/// Returns the user's internal UUID.
pub async fn ensure_user(
    db: &DatabaseConnection,
    input: EnsureUser,
) -> Result<Uuid, EnsureUserError> {
    let auth0_sub = input.auth0_sub.trim().to_string();
    let name = input.name.trim().to_string();
    let email = input.email.trim().to_string();

    if auth0_sub.is_empty() {
        return Err(EnsureUserError::InvalidAuth0Sub);
    }

    validate_name(&name)?;
    validate_email(&email)?;

    let user_id = UserRepo::ensure_user_by_auth0_sub(db, auth0_sub, name, email).await?;

    Ok(user_id)
}
