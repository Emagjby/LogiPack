pub use sea_orm_migration::prelude::*;

mod m2026_01_13_eventstore;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m2026_01_13_eventstore::Migration)]
    }

    fn migration_table_name() -> DynIden {
        Alias::new("eventstore_seaql_migrations").into_iden()
    }
}
