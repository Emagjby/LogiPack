use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

use crate::helpers::setup_app_with_db;

/// Helper: build a POST /ensure-user request
fn ensure_user_request(sub: &str, name: &str, email: &str) -> Request<Body> {
    Request::builder()
        .header("x-dev-secret", "test_secret")
        .header("x-dev-user-sub", sub)
        .header("content-type", "application/json")
        .method(Method::POST)
        .uri("/ensure-user")
        .body(Body::from(
            serde_json::to_vec(&json!({ "name": name, "email": email })).unwrap(),
        ))
        .unwrap()
}

#[tokio::test]
async fn ensure_user_is_idempotent_no_duplicates() {
    let (app, db) = setup_app_with_db().await;

    let sub = "auth0|idempotent-test";

    // First call: creates user
    let res = app
        .oneshot(ensure_user_request(sub, "First Name", "first@example.com"))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Build a fresh router (oneshot consumes the app)
    let state = hub_api::state::AppState {
        db: db.clone(),
        auth_mode: hub_api::config::AuthMode::DevSecret,
    };
    let cfg = hub_api::config::Config {
        host: "127.0.0.1".to_string(),
        port: 3000,
        dev_secret: "test_secret".to_string(),
        auth_mode: hub_api::config::AuthMode::DevSecret,
        auth0_issuer: None,
        auth0_audience: None,
        auth0_jwks_url: None,
        auth0_jwks_path: None,
    };
    let app2 = hub_api::app::router(cfg, state);

    // Second call: same sub, different name/email -> should update, not create duplicate
    let res = app2
        .oneshot(ensure_user_request(
            sub,
            "Updated Name",
            "updated@example.com",
        ))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Verify only one user with this auth0_sub
    use core_data::entity::users;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let users_found = users::Entity::find()
        .filter(users::Column::Auth0Sub.eq(sub))
        .all(&db)
        .await
        .unwrap();

    assert_eq!(users_found.len(), 1, "Should not create duplicate users");

    // Verify name/email were updated
    let user = &users_found[0];
    assert_eq!(user.name, "Updated Name");
    assert_eq!(user.email, Some("updated@example.com".to_string()));
}

#[tokio::test]
async fn ensure_user_idempotent_same_data_succeeds() {
    let (app, db) = setup_app_with_db().await;

    let sub = "auth0|same-data-test";

    // First call
    let res = app
        .oneshot(ensure_user_request(sub, "Same Name", "same@example.com"))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Second call with identical data
    let state = hub_api::state::AppState {
        db: db.clone(),
        auth_mode: hub_api::config::AuthMode::DevSecret,
    };
    let cfg = hub_api::config::Config {
        host: "127.0.0.1".to_string(),
        port: 3000,
        dev_secret: "test_secret".to_string(),
        auth_mode: hub_api::config::AuthMode::DevSecret,
        auth0_issuer: None,
        auth0_audience: None,
        auth0_jwks_url: None,
        auth0_jwks_path: None,
    };
    let app2 = hub_api::app::router(cfg, state);

    let res = app2
        .oneshot(ensure_user_request(sub, "Same Name", "same@example.com"))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Verify still one user
    use core_data::entity::users;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let count = users::Entity::find()
        .filter(users::Column::Auth0Sub.eq(sub))
        .all(&db)
        .await
        .unwrap();
    assert_eq!(count.len(), 1);
}
