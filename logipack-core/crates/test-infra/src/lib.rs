use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tokio::sync::OnceCell;

use core_eventstore_migration::{Migrator as EventstoreMigrator, MigratorTrait};

static MIGRATIONS: OnceCell<()> = OnceCell::const_new();

fn test_database_url() -> String {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost/logipack_test".to_string());

    assert!(
        db_url.contains("test"),
        "Refusing to run integration tests against non-test database"
    );

    db_url
}

async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = test_database_url();

    let mut options = ConnectOptions::new(db_url);
    options
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5));

    Database::connect(options).await
}

async fn run_migrations_once() {
    MIGRATIONS
        .get_or_init(|| async {
            let db = establish_connection().await.unwrap();
            EventstoreMigrator::up(&db, None).await.unwrap();
        })
        .await;
}

/// Returns a fresh `DatabaseConnection` for each test.
///
/// Migrations are guaranteed to run exactly once per test run.
pub async fn test_db() -> DatabaseConnection {
    run_migrations_once().await;
    establish_connection().await.unwrap()
}
