use axum::{body::Body, extract::Request, http::Method};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::helpers::setup_app_with_db;

#[tokio::test]
async fn me_returns_404_when_user_not_provisioned() {
    let (app, _db) = setup_app_with_db().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", "auth0|nonexistent-user")
                .method(Method::GET)
                .uri("/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::NOT_FOUND);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["code"], "user_not_provisioned");
}
