use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::routes;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(routes::health::get_health))
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt as _;
    use tower::ServiceExt as _;

    #[tokio::test]
    async fn health_route_is_wired() {
        let app = router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json, serde_json::json!({"status": "ok"}));
    }
}
