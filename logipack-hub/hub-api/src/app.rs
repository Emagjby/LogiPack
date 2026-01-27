use crate::auth::middleware::{Auth0Config, auth0_jwt_middleware};
use crate::config::{AuthMode, Config};
use crate::state::AppState;
use axum::{Router, routing::get};

use crate::routes;

pub fn router(cfg: Config, state: AppState) -> Router {
    let mut router = Router::new()
        .route("/health", get(routes::health::get_health))
        .nest("/shipments", routes::shipments::router())
        .with_state(state);

    router = match cfg.auth_mode {
        AuthMode::DevSecret => {
            let dev_secret = cfg.dev_secret.clone();
            router.layer(axum::middleware::from_fn(move |req, next| {
                crate::dev_secret::dev_secret_middleware(req, next, dev_secret.clone())
            }))
        }

        AuthMode::Auth0 => {
            let auth_cfg = Auth0Config {
                issuer: cfg.auth0_issuer.clone().expect("AUTH0_ISSUER is required"),
                audience: cfg
                    .auth0_audience
                    .clone()
                    .expect("AUTH0_AUDIENCE is required"),
                jwks_url: cfg.auth0_jwks_url.clone(),
                local_jwks_path: cfg.auth0_jwks_path.clone(),
                local_jwks_json: None,
                jwks_cache_ttl: std::time::Duration::from_secs(60 * 10),
            };

            router.layer(axum::middleware::from_fn(move |req, next| {
                auth0_jwt_middleware(req, next, auth_cfg.clone())
            }))
        }
    };

    router
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt as _;
    use test_infra::test_db;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_requires_dev_secret() {
        let db = test_db().await;
        let app = router(
            test_config(),
            AppState {
                db,
                auth_mode: AuthMode::DevSecret,
            },
        );

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn health_with_dev_secret_is_ok() {
        let db = test_db().await;
        let app = router(
            test_config(),
            AppState {
                db,
                auth_mode: AuthMode::DevSecret,
            },
        );

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .header("x-dev-secret", "test_secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), axum::http::StatusCode::OK);

        let body_bytes = res.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json, serde_json::json!({"status": "ok"}));
    }

    #[tokio::test]
    async fn health_with_wrong_dev_secret_is_unauthorized() {
        let db = test_db().await;
        let app = router(
            test_config(),
            AppState {
                db,
                auth_mode: AuthMode::DevSecret,
            },
        );

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .header("x-dev-secret", "wrong_secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    fn test_config() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 3000,
            dev_secret: "test_secret".to_string(),
            auth_mode: AuthMode::DevSecret,
            auth0_issuer: None,
            auth0_audience: None,
            auth0_jwks_url: None,
            auth0_jwks_path: None,
        }
    }
}
