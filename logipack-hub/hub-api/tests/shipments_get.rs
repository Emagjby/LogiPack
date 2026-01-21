use axum::{body::Body, extract::Request, http::StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use uuid::Uuid;

use core_application::shipments::create::{CreateShipment, create_shipment};
use hub_api::dto::shipments::{ShipmentDetail, ShipmentListItem};

#[allow(dead_code)]
mod helpers;
use helpers::{seed_client, seed_office, setup_app, setup_app_with_admin};

#[tokio::test]
async fn list_shipments_empty() {
    let app = setup_app().await;
    let res = app
        .oneshot(
            Request::builder()
                .uri("/shipments")
                .header("x-dev-secret", "test_secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(body.is_empty());
}

#[tokio::test]
async fn list_shipments_returns_rows() {
    let (app, db, admin) = setup_app_with_admin().await;

    let client = seed_client(&db).await;
    let office = seed_office(&db).await;

    create_shipment(
        &db,
        &admin,
        CreateShipment {
            client_id: client,
            current_office_id: Some(office),
            notes: None,
        },
    )
    .await
    .unwrap();

    let res = app
        .oneshot(
            Request::builder()
                .uri("/shipments")
                .header("x-dev-secret", "test_secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<ShipmentListItem> = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].current_status, "NEW");
}

#[tokio::test]
async fn get_shipment_404() {
    let app = setup_app().await;

    let res = app
        .oneshot(
            Request::builder()
                .uri(format!("/shipments/{}", Uuid::new_v4()))
                .header("x-dev-secret", "test_secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_shipment_returns_detail() {
    let (app, db, admin) = setup_app_with_admin().await;

    let client = seed_client(&db).await;
    let office = seed_office(&db).await;

    let shipment = create_shipment(
        &db,
        &admin,
        CreateShipment {
            client_id: client,
            current_office_id: Some(office),
            notes: Some("hello".into()),
        },
    )
    .await
    .unwrap();

    let res = app
        .oneshot(
            Request::builder()
                .uri(format!("/shipments/{}", shipment))
                .header("x-dev-secret", "test_secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body: ShipmentDetail = serde_json::from_slice(&body).unwrap();

    assert_eq!(body.id.to_string(), shipment.to_string());
    assert_eq!(body.current_status, "NEW");
}
