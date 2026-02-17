use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

use crate::helpers::setup_app_with_db;

#[tokio::test]
async fn ensure_user_rejects_unauthenticated() {
    let (app, _db) = setup_app_with_db().await;

    // No x-dev-secret header → dev secret middleware rejects with 401
    let res = app
        .oneshot(
            Request::builder()
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

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn ensure_user_rejects_wrong_secret() {
    let (app, _db) = setup_app_with_db().await;

    // Wrong x-dev-secret
    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "wrong_secret")
                .header("x-dev-user-sub", "auth0|test")
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

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
