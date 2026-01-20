use core_application::roles::Role;
use core_application::shipments::change_status::change_status;
use core_application::shipments::create::{CreateShipment, create_shipment};
use core_application::{actor::ActorContext, shipments::change_status::ChangeStatus};
use core_data::entity::{clients, employee_offices, employees, offices, users};
use core_domain::shipment::ShipmentStatus;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, DbBackend, Set, Statement};
use test_infra::test_db;
use uuid::Uuid;

async fn cleanup(db: &DatabaseConnection) {
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

async fn seed_client(db: &DatabaseConnection) -> Uuid {
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

async fn seed_office(db: &DatabaseConnection) -> Uuid {
    let id = Uuid::new_v4();

    offices::ActiveModel {
        id: Set(id),
        name: Set("Office".into()),
        city: Set("City".into()),
        address: Set("Address".into()),
    }
    .insert(db)
    .await
    .unwrap();

    id
}

async fn seed_user(db: &DatabaseConnection, user_type: Option<String>) -> Uuid {
    let id = Uuid::new_v4();
    let email = match user_type {
        Some(t) => format!("{}+{}@test.com", t, id),
        None => format!("{}+{}@test.com", "user_any", id),
    };

    users::ActiveModel {
        id: Set(id),
        email: Set(email),
        password_hash: Set("x".into()),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(db)
    .await
    .unwrap();

    id
}

async fn seed_employee(db: &DatabaseConnection, user_id: Uuid) -> Uuid {
    let id = Uuid::new_v4();

    employees::ActiveModel {
        id: Set(id),
        user_id: Set(user_id),
        full_name: Set("Test Employee".into()),
    }
    .insert(db)
    .await
    .unwrap();

    id
}

async fn allow_employee_office(db: &DatabaseConnection, employee_id: Uuid, office_id: Uuid) {
    employee_offices::ActiveModel {
        employee_id: Set(employee_id),
        office_id: Set(office_id),
    }
    .insert(db)
    .await
    .unwrap();
}

async fn admin_actor(db: &DatabaseConnection) -> ActorContext {
    let user_id = seed_user(db, Some("admin".to_string())).await;

    ActorContext {
        user_id,
        sub: "admin".into(),
        roles: vec![Role::Admin],
        employee_id: None,
        allowed_office_ids: vec![],
    }
}

async fn employee_actor(db: &DatabaseConnection, allowed_office_ids: Vec<Uuid>) -> ActorContext {
    let user_id = seed_user(db, Some("employee".to_string())).await;
    let employee_id = seed_employee(db, user_id).await;

    for office_id in &allowed_office_ids {
        allow_employee_office(db, employee_id, *office_id).await;
    }

    ActorContext {
        user_id,
        sub: "employee".into(),
        roles: vec![Role::Employee],
        employee_id: Some(employee_id),
        allowed_office_ids,
    }
}

#[tokio::test]
async fn admin_can_change_status() {
    let db = test_db().await;
    cleanup(&db).await;

    let office = seed_office(&db).await;
    let client = seed_client(&db).await;

    let admin = admin_actor(&db).await;

    let shipment_id = create_shipment(
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

    change_status(
        &db,
        &admin,
        ChangeStatus {
            shipment_id,
            to_status: ShipmentStatus::Accepted,
            to_office_id: Some(office),
            notes: None,
        },
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn employee_cannot_change_status_outside_office() {
    let db = test_db().await;
    cleanup(&db).await;

    let office1 = seed_office(&db).await;
    let office2 = seed_office(&db).await;
    let client = seed_client(&db).await;

    let admin = admin_actor(&db).await;

    let shipment_id = create_shipment(
        &db,
        &admin,
        CreateShipment {
            client_id: client,
            current_office_id: Some(office1),
            notes: None,
        },
    )
    .await
    .unwrap();

    let employee = employee_actor(&db, vec![office1]).await;

    let err = change_status(
        &db,
        &employee,
        ChangeStatus {
            shipment_id,
            to_status: ShipmentStatus::InTransit,
            to_office_id: Some(office2),
            notes: None,
        },
    )
    .await
    .unwrap_err();

    assert!(matches!(
        err,
        core_application::shipments::change_status::ChangeStatusError::Forbidden
    ));
}

#[tokio::test]
async fn employee_can_change_statuts_inside_office() {
    let db = test_db().await;
    cleanup(&db).await;

    let office = seed_office(&db).await;
    let client = seed_client(&db).await;

    let admin = admin_actor(&db).await;

    let shipment_id = create_shipment(
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

    let employee = employee_actor(&db, vec![office]).await;

    change_status(
        &db,
        &employee,
        ChangeStatus {
            shipment_id,
            to_status: ShipmentStatus::Accepted,
            to_office_id: Some(office),
            notes: None,
        },
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn office_hop_only_allowed_when_in_transit() {
    let db = test_db().await;
    cleanup(&db).await;

    let office1 = seed_office(&db).await;
    let office2 = seed_office(&db).await;
    let client = seed_client(&db).await;

    let admin = admin_actor(&db).await;

    let shipment_id = create_shipment(
        &db,
        &admin,
        CreateShipment {
            client_id: client,
            current_office_id: Some(office1),
            notes: None,
        },
    )
    .await
    .unwrap();

    // Move forward to Processed
    change_status(
        &db,
        &admin,
        ChangeStatus {
            shipment_id,
            to_status: ShipmentStatus::Accepted,
            to_office_id: Some(office1),
            notes: None,
        },
    )
    .await
    .unwrap();

    change_status(
        &db,
        &admin,
        ChangeStatus {
            shipment_id,
            to_status: ShipmentStatus::Processed,
            to_office_id: Some(office1),
            notes: None,
        },
    )
    .await
    .unwrap();

    // illegal office hop
    let err = change_status(
        &db,
        &admin,
        ChangeStatus {
            shipment_id,
            to_status: ShipmentStatus::Delivered,
            to_office_id: Some(office1),
            notes: None,
        },
    )
    .await
    .unwrap_err();

    assert!(matches!(
        err,
        core_application::shipments::change_status::ChangeStatusError::Domain(_)
    ));

    // legal office hop
    change_status(
        &db,
        &admin,
        ChangeStatus {
            shipment_id,
            to_status: ShipmentStatus::InTransit,
            to_office_id: Some(office2),
            notes: None,
        },
    )
    .await
    .unwrap();
}
