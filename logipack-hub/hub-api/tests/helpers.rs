use jsonwebtoken::Header;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Router;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use uuid::Uuid;

use core_application::actor::ActorContext;
use test_infra::test_db;

use hub_api::app;
use hub_api::config::{AuthMode, Config};
use hub_api::state::AppState;

pub fn sign_test_jwt(
    kid: &str,
    issuer: &str,
    audience: &str,
    sub: &str,
    private_pem: &str,
) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let claims = json!({
        "sub": sub,
        "iss": issuer,
        "aud": audience,
        "iat": now,
        "nbf": now - 1,
        "exp": now + 3600, // 1 hour expiry
    });

    let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some(kid.to_string());

    jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(private_pem.as_bytes()).unwrap(),
    )
    .unwrap()
}

pub fn test_config() -> Config {
    Config {
        host: "127.0.0.1".to_string(),
        port: 3000,
        dev_secret: "test_secret".to_string(),
        auth_mode: AuthMode::DevSecret,
        auth0_issuer: None,
        auth0_audience: None,
        auth0_jwks_url: None,
        auth0_jwks_path: None,
    }
}

pub async fn cleanup_db(db: &DatabaseConnection) {
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

pub async fn setup_auth0_app() -> axum::Router {
    unsafe {
        std::env::set_var("LOGIPACK_AUTH_MODE", "auth0");
        std::env::set_var("AUTH0_ISSUER", "https://test/");
        std::env::set_var("AUTH0_AUDIENCE", "logipack");
        std::env::set_var(
            "AUTH0_JWKS_PATH",
            format!("{}/tests/fixtures/jwks.json", env!("CARGO_MANIFEST_DIR")),
        );
    }

    let db = test_db().await;
    let state = AppState {
        db,
        auth_mode: AuthMode::Auth0,
    };

    let cfg = Config::from_env();
    app::router(cfg, state)
}

pub async fn setup_app() -> Router {
    let db = test_db().await;

    cleanup_db(&db).await;

    let state = AppState {
        db,
        auth_mode: AuthMode::DevSecret,
    };

    let cfg = test_config();

    app::router(cfg, state)
}

pub async fn setup_app_with_db() -> (Router, DatabaseConnection) {
    let db = test_db().await;

    cleanup_db(&db).await;

    let state = AppState {
        db: db.clone(),
        auth_mode: AuthMode::DevSecret,
    };

    let cfg = test_config();

    let app = app::router(cfg, state);

    (app, db)
}

pub async fn seed_admin_actor(db: &DatabaseConnection) -> ActorContext {
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
        auth0_sub: Set(None),
        password_hash: Set("x".into()),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(db)
    .await
    .unwrap();

    // role row (admin)
    roles::ActiveModel {
        id: Set(role_id),
        name: Set("admin".into()),
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

    let email = format!("admin+{}@test.com", user_id);

    ActorContext {
        user_id,
        sub: email.clone(),
        roles: vec![Role::Admin],
        employee_id: None,
        allowed_office_ids: vec![],
    }
}

pub async fn seed_employee(db: &DatabaseConnection) -> ActorContext {
    use core_application::roles::Role;
    use core_data::entity::{roles, user_roles, users};
    use sea_orm::{ActiveModelTrait, Set};
    use uuid::Uuid;

    let user_id = Uuid::new_v4();
    let role_id = Uuid::new_v4();

    // user
    users::ActiveModel {
        id: Set(user_id),
        email: Set(format!("employee+{}@test.com", user_id)),
        password_hash: Set("x".into()),
        auth0_sub: Set(None),
        created_at: Set(chrono::Utc::now().into()),
    }
    .insert(db)
    .await
    .unwrap();

    // role row (admin)
    roles::ActiveModel {
        id: Set(role_id),
        name: Set("employee".into()),
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

    let email = format!("employee+{}@test.com", user_id);

    ActorContext {
        user_id,
        sub: email.clone(),
        roles: vec![Role::Employee],
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

pub async fn setup_app_with_admin() -> (Router, DatabaseConnection, ActorContext) {
    let db = test_db().await;

    cleanup_db(&db).await;

    let state = AppState {
        db: db.clone(),
        auth_mode: AuthMode::DevSecret,
    };

    let cfg = test_config();

    let app = app::router(cfg, state);

    let admin_actor = seed_admin_actor(&db).await;

    (app, db, admin_actor)
}
