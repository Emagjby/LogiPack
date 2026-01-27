use axum::{body::Body, extract::Request};
use tower::ServiceExt;

use crate::helpers::{seed_auth0_user, setup_auth0_app, setup_auth0_app_with_db, sign_test_jwt};
use std::env;

#[allow(dead_code)]
mod helpers;

#[tokio::test]
async fn auth0_sub_resolves_actor() {
    let (app, db) = setup_auth0_app_with_db().await;

    let sub = "auth0|user123";
    seed_auth0_user(&db, sub).await;

    let private = env::var("TEST_AUTH0_PRIVATE_PEM").unwrap();

    let token = sign_test_jwt(
        "vPGrStQtI1pBCs8y+UqMe7vR/S90cOiQQJy3BKyEnJI=",
        "https://test/",
        "logipack",
        sub,
        &private,
    );

    let res = app
        .oneshot(
            Request::builder()
                .uri("/shipments")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn auth0_unknown_sub_is_401() {
    let app = setup_auth0_app().await;

    let private = env::var("TEST_AUTH0_PRIVATE_PEM").unwrap();

    let token = sign_test_jwt(
        "vPGrStQtI1pBCs8y+UqMe7vR/S90cOiQQJy3BKyEnJI=",
        "https://test/",
        "logipack",
        "auth0|does_not_exist",
        &private,
    );

    let res = app
        .oneshot(
            Request::builder()
                .uri("/shipments")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
}
