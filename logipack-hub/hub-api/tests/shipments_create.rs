use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
};
use http_body_util::BodyExt;
use hub_api::dto::shipments::CreateShipmentResponse;
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::helpers::{
    seed_client, seed_employee, seed_office, setup_app_with_admin, setup_app_with_db,
};

#[allow(dead_code)]
pub mod helpers;

#[tokio::test]
async fn admin_can_create_shipment() {
    let (app, db, admin) = setup_app_with_admin().await;

    let client = seed_client(&db).await;
    let office = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/shipments")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "client_id": client,
                        "current_office_id": office,
                        "notes": "hello"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body: CreateShipmentResponse = serde_json::from_slice(&body).unwrap();

    assert_ne!(body.shipment_id, Uuid::nil());
}

#[tokio::test]
async fn employee_cannot_create_shipment() {
    let (app, db) = setup_app_with_db().await;

    let client = seed_client(&db).await;
    let office = seed_office(&db).await;
    let employee = seed_employee(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", employee.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/shipments")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "client_id": client,
                        "current_office_id": office,
                        "notes": "hello"
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
async fn create_shipment_invalid_json() {
    let (app, _db, admin) = setup_app_with_admin().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/shipments")
                .body(Body::from("{invalid"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_shipment_invalid_uuid() {
    let (app, db, admin) = setup_app_with_admin().await;

    let office = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/shipments")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "client_id": "not-a-uuid",
                        "current_office_id": office
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn create_shipment_invalid_client() {
    let (app, db, admin) = setup_app_with_admin().await;

    let office = seed_office(&db).await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/shipments")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "client_id": Uuid::new_v4(),
                        "current_office_id": office
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
async fn create_shipment_missing_client_id() {
    let (app, _db, admin) = setup_app_with_admin().await;

    let res = app
        .oneshot(
            Request::builder()
                .header("x-dev-secret", "test_secret")
                .header("x-dev-user-sub", admin.sub.clone())
                .header("content-type", "application/json")
                .method(Method::POST)
                .uri("/shipments")
                .body(Body::from(
                    serde_json::to_vec(&json!({ "current_office_id": Uuid::new_v4() })).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
