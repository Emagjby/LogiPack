use crate::config::Config;
use crate::state::AppState;
use axum::{Router, routing::get};

use crate::routes;

pub fn router(cfg: Config, state: AppState) -> Router {
    let dev_secret = cfg.dev_secret.clone();

    Router::new()
        .route("/health", get(routes::health::get_health))
        .nest("/shipments", routes::shipments::router())
        .layer(axum::middleware::from_fn(move |req, next| {
            crate::dev_secret::dev_secret_middleware(req, next, dev_secret.clone())
        }))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt as _;
    use sea_orm::{ConnectionTrait, DatabaseConnection, Set};
    use test_infra::test_db;

    async fn cleanup_db(db: &DatabaseConnection) {
        let _ = db
            .execute_unprepared("TRUNCATE TABLE user_roles, users, roles RESTART IDENTITY CASCADE")
            .await;
    }
    use tower::ServiceExt as _;

    #[tokio::test]
    async fn health_requires_dev_secret() {
        let db = test_db().await;
        let app = router(test_config(), AppState { db });

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
        let app = router(test_config(), AppState { db });

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
        let app = router(test_config(), AppState { db });

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

    #[tokio::test]
    async fn whoami_missing_sub_is_401() {
        let db = test_db().await;
        cleanup_db(&db).await;
        let app = test_router(test_config(), AppState { db });

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/__test/whoami")
                    .header("x-dev-secret", "test_secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn whoami_unknown_user_is_401() {
        let db = test_db().await;
        cleanup_db(&db).await;
        let app = test_router(test_config(), AppState { db });

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/__test/whoami")
                    .header("x-dev-secret", "test_secret")
                    .header("x-dev-user-sub", "ghost@test.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn whoami_admin_resolves() {
        let db = test_db().await;
        cleanup_db(&db).await;
        let (_id, email) = seed_admin_user(&db).await;

        let app = test_router(test_config(), AppState { db });

        let res = app
            .oneshot(
                Request::builder()
                    .uri("/__test/whoami")
                    .header("x-dev-secret", "test_secret")
                    .header("x-dev-user-sub", email)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }

    async fn whoami(actor: core_application::actor::ActorContext) -> axum::Json<serde_json::Value> {
        use serde_json::json;

        axum::Json(json!({
            "user_id": actor.user_id,
            "roles": actor.roles.iter().map(|r| format!("{:?}", r)).collect::<Vec<_>>(),
            "employee_id": actor.employee_id,
            "offices": actor.allowed_office_ids,
        }))
    }

    async fn seed_admin_user(db: &DatabaseConnection) -> (uuid::Uuid, String) {
        use sea_orm::ActiveModelTrait;
        use uuid::Uuid;

        let user_id = Uuid::new_v4();
        let email = format!("admin+{user_id}@test.com");

        core_data::entity::users::ActiveModel {
            id: Set(user_id),
            email: Set(email.clone()),
            password_hash: Set("x".into()),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();

        let role_id = Uuid::new_v4();

        core_data::entity::roles::ActiveModel {
            id: Set(role_id),
            name: Set("admin".to_string()),
        }
        .insert(db)
        .await
        .unwrap();

        core_data::entity::user_roles::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
        }
        .insert(db)
        .await
        .unwrap();

        (user_id, email)
    }

    fn test_config() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 3000,
            dev_secret: "test_secret".to_string(),
        }
    }

    pub fn test_router(cfg: Config, state: AppState) -> Router {
        let dev_secret = cfg.dev_secret.clone();

        Router::new()
            .route("/__test/whoami", get(whoami))
            .layer(axum::middleware::from_fn(move |req, next| {
                crate::dev_secret::dev_secret_middleware(req, next, dev_secret.clone())
            }))
            .with_state(state)
    }
}
