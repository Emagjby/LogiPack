use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
};
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

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

/// Helper: rebuild the router from an existing DB connection (oneshot consumes the app).
fn rebuild_app(db: sea_orm::DatabaseConnection) -> axum::Router {
    let state = hub_api::state::AppState {
        db,
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
    hub_api::app::router(cfg, state)
}

/// Case A: Existing user with same email but `auth0_sub = NULL`.
/// Calling `/ensure-user` should link the auth0_sub and NOT create a second row.
#[tokio::test]
async fn ensure_user_links_auth0_sub_when_email_exists_with_null_sub() {
    let (app, db) = setup_app_with_db().await;

    let email = "existing@example.com";
    let existing_user_id = Uuid::new_v4();

    // Seed a user with matching email but no auth0_sub (e.g. local/seeded user)
    core_data::entity::users::ActiveModel {
        id: Set(existing_user_id),
        name: Set("Old Name".into()),
        email: Set(Some(email.to_string())),
        auth0_sub: Set(None),
        password_hash: Set(Some("hashed".into())),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(&db)
    .await
    .unwrap();

    let new_sub = "auth0|link-me-123";

    // Call ensure-user with this email + a new auth0_sub
    let res = app
        .oneshot(ensure_user_request(new_sub, "New Name", email))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Verify: still only ONE user with this email
    let users = core_data::entity::users::Entity::find()
        .filter(core_data::entity::users::Column::Email.eq(Some(email.to_string())))
        .all(&db)
        .await
        .unwrap();

    assert_eq!(users.len(), 1, "Should not create a duplicate user row");

    // Verify: the existing row was linked
    let user = &users[0];
    assert_eq!(user.id, existing_user_id, "Should reuse the existing user");
    assert_eq!(
        user.auth0_sub,
        Some(new_sub.to_string()),
        "auth0_sub should be linked"
    );
    assert_eq!(user.name, "New Name", "Name should be updated");
}

/// Case A (variant): Ensure name is updated when linking.
#[tokio::test]
async fn ensure_user_link_updates_name() {
    let (app, db) = setup_app_with_db().await;

    let email = "link-update@example.com";
    let existing_user_id = Uuid::new_v4();

    core_data::entity::users::ActiveModel {
        id: Set(existing_user_id),
        name: Set("Original Name".into()),
        email: Set(Some(email.to_string())),
        auth0_sub: Set(None),
        password_hash: Set(None),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(&db)
    .await
    .unwrap();

    let res = app
        .oneshot(ensure_user_request(
            "auth0|update-name",
            "Updated Name",
            email,
        ))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let user = core_data::entity::users::Entity::find_by_id(existing_user_id)
        .one(&db)
        .await
        .unwrap()
        .expect("user should exist");

    assert_eq!(user.name, "Updated Name");
    assert_eq!(user.auth0_sub, Some("auth0|update-name".to_string()));
}

/// Case B: Existing user with same email but a DIFFERENT auth0_sub.
/// Calling `/ensure-user` should return 409 Conflict.
#[tokio::test]
async fn ensure_user_returns_409_when_email_linked_to_different_sub() {
    let (app, db) = setup_app_with_db().await;

    let email = "conflict@example.com";

    // Seed user with email AND a different auth0_sub
    core_data::entity::users::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set("Existing User".into()),
        email: Set(Some(email.to_string())),
        auth0_sub: Set(Some("auth0|original-owner".to_string())),
        password_hash: Set(None),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(&db)
    .await
    .unwrap();

    // Try ensure-user with a DIFFERENT sub but the same email
    let res = app
        .oneshot(ensure_user_request(
            "auth0|different-user",
            "Another User",
            email,
        ))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CONFLICT);

    // Verify error body
    let body = axum::body::to_bytes(res.into_body(), 1024 * 64)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["code"], "email_already_linked");
}

/// Case C: Existing user with same auth0_sub updates name/email and stays one row.
/// (This is essentially the existing idempotent test, but we verify email update too.)
#[tokio::test]
async fn ensure_user_updates_existing_user_by_sub() {
    let (app, db) = setup_app_with_db().await;

    let sub = "auth0|update-test";

    // First call: create
    let res = app
        .oneshot(ensure_user_request(sub, "First Name", "first@example.com"))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Second call: update name and email
    let app2 = rebuild_app(db.clone());
    let res = app2
        .oneshot(ensure_user_request(
            sub,
            "Updated Name",
            "updated@example.com",
        ))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Verify: only one user row
    let users = core_data::entity::users::Entity::find()
        .filter(core_data::entity::users::Column::Auth0Sub.eq(sub))
        .all(&db)
        .await
        .unwrap();

    assert_eq!(users.len(), 1, "Should not create duplicate users");
    assert_eq!(users[0].name, "Updated Name");
    assert_eq!(users[0].email, Some("updated@example.com".to_string()));
}

/// Ensure that email comparison is case-insensitive:
/// a seeded user with a lowercase email ("user@example.com") should be found
/// when ensure-user is called with a mixed-case email ("User@Example.com").
#[tokio::test]
async fn ensure_user_email_linking_is_case_insensitive() {
    let (app, db) = setup_app_with_db().await;

    let existing_user_id = Uuid::new_v4();

    // Seed user with lowercase email, no auth0_sub
    core_data::entity::users::ActiveModel {
        id: Set(existing_user_id),
        name: Set("Mixed Case".into()),
        email: Set(Some("user@example.com".to_string())),
        auth0_sub: Set(None),
        password_hash: Set(None),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(&db)
    .await
    .unwrap();

    // Call ensure-user with different casing
    let res = app
        .oneshot(ensure_user_request(
            "auth0|case-test",
            "Mixed Case",
            "User@Example.com",
        ))
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Verify: the existing row was linked (not a new one created)
    let user = core_data::entity::users::Entity::find_by_id(existing_user_id)
        .one(&db)
        .await
        .unwrap()
        .expect("user should exist");

    assert_eq!(user.auth0_sub, Some("auth0|case-test".to_string()));
}
