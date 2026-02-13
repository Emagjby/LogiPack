use axum::{body::Body, extract::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use uuid::Uuid;

use hub_api::dto::offices::ListOfficesResponse;

use crate::helpers::{seed_employee, seed_office, setup_app_with_admin, setup_app_with_db};

#[tokio::test]
async fn list_offices_empty() {
    let (app, _db, admin) = setup_app_with_admin().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .uri("/admin/offices")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body: ListOfficesResponse = serde_json::from_slice(&body).unwrap();

    assert!(body.offices.is_empty());
}

#[tokio::test]
async fn list_offices_returns_rows() {
    let (app, db, admin) = setup_app_with_admin().await;

    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .uri("/admin/offices")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body: ListOfficesResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(body.offices.len(), 1);
    assert_eq!(body.offices[0].id, office_id.to_string());
}

#[tokio::test]
async fn employee_cannot_list_offices() {
    let (app, db) = setup_app_with_db().await;
    let employee = seed_employee(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", employee.sub.clone())
                .uri("/admin/offices")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn no_role_cannot_list_offices() {
    let (app, _db) = setup_app_with_db().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", Uuid::new_v4().to_string())
                .uri("/admin/offices")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
