use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
};
use tower::ServiceExt;
use uuid::Uuid;

use crate::helpers::{seed_employee, seed_office, setup_app_with_admin, setup_app_with_db};

#[tokio::test]
async fn admin_can_delete_office() {
    let (app, db, admin) = setup_app_with_admin().await;
    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri(format!("/admin/offices/{}", office_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn admin_delete_office_invalid_uuid() {
    let (app, _db, admin) = setup_app_with_admin().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri("/admin/offices/not-a-uuid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn admin_delete_office_not_found() {
    let (app, _db, admin) = setup_app_with_admin().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri(format!("/admin/offices/{}", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn employee_cannot_delete_office() {
    let (app, db) = setup_app_with_db().await;
    let employee = seed_employee(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", employee.sub.clone())
                .method(Method::DELETE)
                .uri(format!("/admin/offices/{}", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn no_role_cannot_delete_office() {
    let (app, _db) = setup_app_with_db().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", Uuid::new_v4().to_string())
                .method(Method::DELETE)
                .uri(format!("/admin/offices/{}", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
