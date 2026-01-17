use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tokio::sync::OnceCell;

use core_data_migration::{Migrator as CoreDataMigrator, MigratorTrait};
use core_eventstore_migration::Migrator as EventstoreMigrator;

static MIGRATIONS: OnceCell<()> = OnceCell::const_new();

fn test_database_url() -> String {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost/logipack_test".to_string());

    assert!(
        db_url.contains("test"),
        "Refusing to run integration tests against non-test database. DATABASE_URL={db_url}"
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
            let db = establish_connection()
                .await
                .expect("failed to connect to test database");

            CoreDataMigrator::up(&db, None)
                .await
                .expect("core-data migrations failed");

            // Eventstore migrations contain Postgres DDL that cannot run inside a transaction.
            // SeaORM runs Postgres migrations inside a transaction by default; when Postgres
            // rejects one statement, the transaction becomes aborted (25P02), and subsequent
            // statements all fail. Core-data migrations stay transactional for safety; only the
            // eventstore migrator is executed non-transactionally.
            // Run via the migrator so it stays idempotent (pending-only) and
            // records entries in the eventstore migration history table.
            EventstoreMigrator::up(&db, None)
                .await
                .expect("eventstore migrations failed");
        })
        .await;
}

/// Returns a fresh `DatabaseConnection` for each test.
///
/// Migrations are guaranteed to run exactly once per test run.
pub async fn test_db() -> DatabaseConnection {
    run_migrations_once().await;
    establish_connection()
        .await
        .expect("failed to establish test database connection")
}
