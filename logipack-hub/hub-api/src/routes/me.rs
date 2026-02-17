use axum::{Json, Router, extract::State, routing::get};

use crate::{dto::me::MeResponse, error::ApiError, state::AppState};

use super::auth_sub::extract_sub;

pub fn router() -> Router<AppState> {
    Router::new().route("/me", get(me_handler))
}

async fn me_handler(
    State(state): State<AppState>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<Json<MeResponse>, ApiError> {
    let (parts, _body) = request.into_parts();

    let sub = extract_sub(&parts, state.auth_mode)?;

    let role = core_application::users::me::get_me_role(&state.db, &sub).await?;

    Ok(Json(MeResponse { role }))
}
