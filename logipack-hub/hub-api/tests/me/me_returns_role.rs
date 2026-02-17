use axum::{body::Body, extract::Request, http::Method};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::helpers::{seed_auth0_user, setup_app_with_db};

#[tokio::test]
async fn me_returns_role_for_existing_user() {
    let (app, db) = setup_app_with_db().await;

    let sub = "auth0|me-test";
    seed_auth0_user(&db, sub).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", sub)
                .method(Method::GET)
                .uri("/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["role"], "admin");
}
