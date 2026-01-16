use crate::config::Config;
use axum::{routing::get, Router};

use crate::routes;

pub fn router(cfg: Config) -> Router {
    let dev_secret = cfg.dev_secret.clone();

    Router::new()
        .route("/health", get(routes::health::get_health))
        .layer(axum::middleware::from_fn(move |req, next| {
            crate::dev_secret::dev_secret_middleware(req, next, dev_secret.clone())
        }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt as _;
    use tower::ServiceExt as _;

    #[tokio::test]
    async fn health_requires_dev_secret() {
        let cfg = Config {
            host: "127.0.0.1".to_string(),
            port: 3000,
            dev_secret: "test_secret".to_string(),
        };

        let app = router(cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn health_with_dev_secret_is_ok() {
        let cfg = Config {
            host: "127.0.0.1".to_string(),
            port: 3000,
            dev_secret: "test_secret".to_string(),
        };

        let app = router(cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .header("x-dev-secret", "test_secret")
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

    #[tokio::test]
    async fn health_with_wrong_dev_secret_is_unauthorized() {
        let cfg = Config {
            host: "127.0.0.1".to_string(),
            port: 3000,
            dev_secret: "test_secret".to_string(),
        };

        let app = router(cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .header("x-dev-secret", "wrong_secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }
}
