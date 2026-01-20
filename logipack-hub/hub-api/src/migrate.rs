use core_data_migration::Migrator as CoreDataMigrator;
use core_eventstore_migration::Migrator as CoreEventstoreMigrator;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

pub async fn migrate(db: &DatabaseConnection) {
    CoreDataMigrator::up(db, None)
        .await
        .expect("core-data migrations");

    CoreEventstoreMigrator::up(db, None)
        .await
        .expect("eventstore migrations");
}
