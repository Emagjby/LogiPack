use crate::{auth::claims::Claims, config::AuthMode, error::ApiError};

/// Extracts the auth0 `sub` from the verified JWT claims or dev header.
///
/// Unlike `ActorContext`, this does NOT require the user to already exist
/// in the database — it only reads the identity from the auth layer.
pub fn extract_sub(
    parts: &axum::http::request::Parts,
    auth_mode: AuthMode,
) -> Result<String, ApiError> {
    match auth_mode {
        AuthMode::DevSecret => parts
            .headers
            .get("x-dev-user-sub")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| ApiError::bad_request("missing_sub", "Missing x-dev-user-sub header")),

        AuthMode::Auth0 => parts
            .extensions
            .get::<Claims>()
            .map(|c| c.sub.clone())
            .ok_or_else(|| ApiError::bad_request("missing_sub", "Missing JWT claims")),
    }
}
