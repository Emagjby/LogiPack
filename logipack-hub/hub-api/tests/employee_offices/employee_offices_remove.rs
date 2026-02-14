use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::helpers::{
    seed_employee, seed_employee_record, seed_office, setup_app_with_admin, setup_app_with_db,
};

#[tokio::test]
async fn admin_can_remove_office_assignment() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;
    let office_id = seed_office(&db).await;

    // Assign first
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri(format!("/admin/employees/{}/offices", employee_id))
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "office_id": office_id.to_string(),
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Remove
    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri(format!(
                    "/admin/employees/{}/offices/{}",
                    employee_id, office_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn remove_nonexistent_assignment_is_idempotent() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;
    let office_id = seed_office(&db).await;

    // Remove without prior assign
    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri(format!(
                    "/admin/employees/{}/offices/{}",
                    employee_id, office_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn employee_cannot_remove_office() {
    let (app, db) = setup_app_with_db().await;
    let employee = seed_employee(&db).await;
    let employee_id = seed_employee_record(&db).await;
    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", employee.sub.clone())
                .method(Method::DELETE)
                .uri(format!(
                    "/admin/employees/{}/offices/{}",
                    employee_id, office_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn no_role_cannot_remove_office() {
    let (app, db) = setup_app_with_db().await;
    let employee_id = seed_employee_record(&db).await;
    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", Uuid::new_v4().to_string())
                .method(Method::DELETE)
                .uri(format!(
                    "/admin/employees/{}/offices/{}",
                    employee_id, office_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn remove_invalid_employee_uuid() {
    let (app, db, admin) = setup_app_with_admin().await;
    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri(format!("/admin/employees/not-a-uuid/offices/{}", office_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn remove_invalid_office_uuid() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri(format!(
                    "/admin/employees/{}/offices/not-a-uuid",
                    employee_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn remove_employee_not_found() {
    let (app, db, admin) = setup_app_with_admin().await;
    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri(format!(
                    "/admin/employees/{}/offices/{}",
                    Uuid::new_v4(),
                    office_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn remove_office_not_found() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .method(Method::DELETE)
                .uri(format!(
                    "/admin/employees/{}/offices/{}",
                    employee_id,
                    Uuid::new_v4()
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
