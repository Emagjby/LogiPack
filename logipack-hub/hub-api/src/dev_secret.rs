use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn dev_secret_middleware(
    req: Request<Body>,
    next: Next,
    expected_secret: String,
) -> Response {
    let provided = req
        .headers()
        .get("x-dev-secret")
        .and_then(|v| v.to_str().ok());

    match provided {
        Some(secret) if secret == expected_secret => next.run(req).await,
        _ => StatusCode::UNAUTHORIZED.into_response(),
    }
}
