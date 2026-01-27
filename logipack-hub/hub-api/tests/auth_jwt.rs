use std::{env, time::SystemTime};

use axum::{body::Body, extract::Request};
use jsonwebtoken::Header;
use serde_json::json;
use tower::ServiceExt;

use crate::helpers::{setup_auth0_app, sign_test_jwt};

#[allow(dead_code)]
pub mod helpers;

#[tokio::test]
async fn auth0_valid_token_allows_request() {
    let app = setup_auth0_app().await;

    let private = env::var("TEST_AUTH0_PRIVATE_PEM").unwrap();

    let token = sign_test_jwt(
        "vPGrStQtI1pBCs8y+UqMe7vR/S90cOiQQJy3BKyEnJI=",
        "https://test/",
        "logipack",
        "user|123",
        &private,
    );

    let res = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn auth0_missing_token_is_401() {
    let app = setup_auth0_app().await;

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
async fn auth0_wrong_audience_is_401() {
    let app = setup_auth0_app().await;

    let private = env::var("TEST_AUTH0_PRIVATE_PEM").unwrap();

    let token = sign_test_jwt(
        "vPGrStQtI1pBCs8y+UqMe7vR/S90cOiQQJy3BKyEnJI=",
        "https://test/",
        "wrong_audience",
        "user|123",
        &private,
    );

    let res = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth0_expired_token_is_401() {
    let app = setup_auth0_app().await;

    let private = env::var("TEST_AUTH0_PRIVATE_PEM").unwrap();

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = json!({
        "iss": "https://test/",
        "aud": "logipack",
        "sub": "user|123",
        "exp": now - 10,
    });

    let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some("vPGrStQtI1pBCs8y+UqMe7vR/S90cOiQQJy3BKyEnJI=".to_string());

    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(private.as_bytes()).unwrap(),
    )
    .unwrap();

    let res = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth0_unknown_kid_is_401() {
    let app = setup_auth0_app().await;

    let private = env::var("TEST_AUTH0_PRIVATE_PEM").unwrap();

    let token = sign_test_jwt(
        "unknown_kid",
        "https://test/",
        "logipack",
        "user|123",
        &private,
    );

    let res = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
}
