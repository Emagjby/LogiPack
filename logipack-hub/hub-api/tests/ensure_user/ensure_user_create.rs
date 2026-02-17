use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

use crate::helpers::setup_app_with_db;

#[tokio::test]
async fn ensure_user_creates_user_when_missing() {
    let (app, db) = setup_app_with_db().await;

    let sub = "auth0|new-user-12345";

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", sub)
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/ensure-user")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "name": "Jane Doe",
                        "email": "jane@example.com"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // Verify user was created in DB
    use core_data::entity::users;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let user = users::Entity::find()
        .filter(users::Column::Auth0Sub.eq(sub))
        .one(&db)
        .await
        .unwrap();

    assert!(user.is_some(), "User should have been created");
    let user = user.unwrap();
    assert_eq!(user.name, "Jane Doe");
    assert_eq!(user.email, Some("jane@example.com".to_string()));
}

#[tokio::test]
async fn ensure_user_rejects_invalid_name() {
    let (app, _db) = setup_app_with_db().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", "auth0|test")
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/ensure-user")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "name": "A",
                        "email": "valid@example.com"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn ensure_user_rejects_invalid_email() {
    let (app, _db) = setup_app_with_db().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", "auth0|test")
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/ensure-user")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "name": "Valid Name",
                        "email": "not-an-email"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
