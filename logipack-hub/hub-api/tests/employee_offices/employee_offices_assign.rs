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
async fn admin_can_assign_office_to_employee() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;
    let office_id = seed_office(&db).await;

    let res = app
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
}

#[tokio::test]
async fn assign_same_office_twice_returns_conflict() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;
    let office_id = seed_office(&db).await;

    // First assign
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

    // Second assign (conflict)
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

    assert_eq!(res.status(), StatusCode::CONFLICT);

    // Verify only one office in list
    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .uri(format!("/admin/employees/{}/offices", employee_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = http_body_util::BodyExt::collect(res.into_body())
        .await
        .unwrap()
        .to_bytes();
    let body: hub_api::dto::employee_offices::ListEmployeeOfficesResponse =
        serde_json::from_slice(&body).unwrap();
    assert_eq!(body.office_ids.len(), 1);
}

#[tokio::test]
async fn employee_cannot_assign_office() {
    let (app, db) = setup_app_with_db().await;
    let employee = seed_employee(&db).await;
    let employee_id = seed_employee_record(&db).await;
    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", employee.sub.clone())
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

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn no_role_cannot_assign_office() {
    let (app, db) = setup_app_with_db().await;
    let employee_id = seed_employee_record(&db).await;
    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", Uuid::new_v4().to_string())
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

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn assign_invalid_employee_uuid() {
    let (app, _db, admin) = setup_app_with_admin().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/admin/employees/not-a-uuid/offices")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "office_id": Uuid::new_v4().to_string(),
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
async fn assign_invalid_office_id_in_body() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri(format!("/admin/employees/{}/offices", employee_id))
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "office_id": "not-a-uuid",
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
async fn assign_employee_not_found() {
    let (app, db, admin) = setup_app_with_admin().await;
    let office_id = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri(format!("/admin/employees/{}/offices", Uuid::new_v4()))
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

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn assign_office_not_found() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri(format!("/admin/employees/{}/offices", employee_id))
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "office_id": Uuid::new_v4().to_string(),
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
