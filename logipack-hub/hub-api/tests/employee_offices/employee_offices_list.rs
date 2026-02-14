use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use hub_api::dto::employee_offices::ListEmployeeOfficesResponse;

use crate::helpers::{
    seed_employee, seed_employee_record, seed_office, setup_app_with_admin, setup_app_with_db,
};

#[tokio::test]
async fn admin_can_list_employee_offices() {
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

    // List
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

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body: ListEmployeeOfficesResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.office_ids.len(), 1);
    assert_eq!(body.office_ids[0], office_id.to_string());
}

#[tokio::test]
async fn list_empty_offices() {
    let (app, db, admin) = setup_app_with_admin().await;
    let employee_id = seed_employee_record(&db).await;

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

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body: ListEmployeeOfficesResponse = serde_json::from_slice(&body).unwrap();
    assert!(body.office_ids.is_empty());
}

#[tokio::test]
async fn employee_cannot_list_offices() {
    let (app, db) = setup_app_with_db().await;
    let employee = seed_employee(&db).await;
    let employee_id = seed_employee_record(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", employee.sub.clone())
                .uri(format!("/admin/employees/{}/offices", employee_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn no_role_cannot_list_offices() {
    let (app, db) = setup_app_with_db().await;
    let employee_id = seed_employee_record(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", Uuid::new_v4().to_string())
                .uri(format!("/admin/employees/{}/offices", employee_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn list_invalid_employee_uuid() {
    let (app, _db, admin) = setup_app_with_admin().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .uri("/admin/employees/not-a-uuid/offices")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn list_employee_not_found() {
    let (app, _db, admin) = setup_app_with_admin().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .uri(format!("/admin/employees/{}/offices", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
