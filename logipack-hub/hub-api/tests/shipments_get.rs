use axum::{Router, body::Body, extract::Request, http::StatusCode};
use core_application::{
    actor::ActorContext,
    shipments::create::{CreateShipment, create_shipment},
};
use http_body_util::BodyExt;
use hub_api::state::AppState;
use hub_api::{app, dto::shipments::ShipmentListItem};
use hub_api::{config::Config, dto::shipments::ShipmentDetail};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, sqlx::types::chrono};
use test_infra::test_db;
use tower::ServiceExt;
use uuid::Uuid;

fn test_config() -> Config {
    Config {
        host: "127.0.0.1".to_string(),
        port: 3000,
        dev_secret: "test_secret".to_string(),
    }
}

async fn cleanup_db(db: &DatabaseConnection) {
    let tables = [
        "shipment_status_history",
        "shipments",
        "employee_offices",
        "employees",
        "user_roles",
        "users",
        "clients",
        "roles",
        "offices",
        "packages",
        "streams",
    ];

    for t in tables {
        db.execute(Statement::from_string(
            DbBackend::Postgres,
            format!("DELETE FROM {}", t),
        ))
        .await
        .unwrap();
    }
}

async fn setup_app() -> Router {
    let db = test_db().await;

    cleanup_db(&db).await;

    let state = AppState { db };

    let cfg = test_config();

    app::router(cfg, state)
}

async fn seed_admin_actor(db: &DatabaseConnection) -> ActorContext {
    use core_application::roles::Role;
    use core_data::entity::{roles, user_roles, users};
    use sea_orm::{ActiveModelTrait, Set};
    use uuid::Uuid;

    let user_id = Uuid::new_v4();
    let role_id = Uuid::new_v4();

    // user
    users::ActiveModel {
        id: Set(user_id),
        email: Set(format!("admin+{}@test.com", user_id)),
        password_hash: Set("x".into()),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(db)
    .await
    .unwrap();

    // role row (ADMIN)
    roles::ActiveModel {
        id: Set(role_id),
        name: Set("ADMIN".into()),
    }
    .insert(db)
    .await
    .unwrap();

    // user_roles link
    user_roles::ActiveModel {
        user_id: Set(user_id),
        role_id: Set(role_id),
    }
    .insert(db)
    .await
    .unwrap();

    ActorContext {
        user_id,
        sub: "admin".into(),
        roles: vec![Role::Admin],
        employee_id: None,
        allowed_office_ids: vec![],
    }
}

pub async fn seed_client(db: &DatabaseConnection) -> Uuid {
    use core_data::entity::clients;
    use sea_orm::{ActiveModelTrait, Set};
    use uuid::Uuid;

    let id = Uuid::new_v4();

    clients::ActiveModel {
        id: Set(id),
        name: Set("Test Client".into()),
        phone: Set(None),
        email: Set(None),
    }
    .insert(db)
    .await
    .unwrap();

    id
}

pub async fn seed_office(db: &DatabaseConnection) -> Uuid {
    use core_data::entity::offices;
    use sea_orm::{ActiveModelTrait, Set};
    use uuid::Uuid;

    let id = Uuid::new_v4();

    offices::ActiveModel {
        id: Set(id),
        name: Set("Main Office".into()),
        city: Set("Test City".into()),
        address: Set("Test Address".into()),
    }
    .insert(db)
    .await
    .unwrap();

    id
}

async fn setup_app_with_admin() -> (Router, DatabaseConnection, ActorContext) {
    let db = test_db().await;

    cleanup_db(&db).await;

    let state = AppState { db: db.clone() };

    let cfg = test_config();

    let app = app::router(cfg, state);

    let admin_actor = seed_admin_actor(&db).await;

    (app, db, admin_actor)
}

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
