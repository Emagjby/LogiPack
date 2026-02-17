use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{dto::ensure_user::EnsureUserRequest, error::ApiError, state::AppState};

use super::auth_sub::extract_sub;

pub fn router() -> Router<AppState> {
    Router::new().route("/ensure-user", post(ensure_user_handler))
}

async fn ensure_user_handler(
    State(state): State<AppState>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<StatusCode, ApiError> {
    let (parts, body) = request.into_parts();

    let sub = extract_sub(&parts, state.auth_mode)?;

    let Json(payload): Json<EnsureUserRequest> =
        axum::Json::from_bytes(&axum::body::to_bytes(body, 1024 * 64).await.map_err(|_| {
            ApiError::bad_request("invalid_body", "Request body too large or unreadable")
        })?)
        .map_err(|e| ApiError::bad_request("invalid_json", e.to_string()))?;

    let input = core_application::users::ensure_user::EnsureUser {
        auth0_sub: sub,
        name: payload.name,
        email: payload.email,
    };

    core_application::users::ensure_user::ensure_user(&state.db, input).await?;

    Ok(StatusCode::NO_CONTENT)
}
