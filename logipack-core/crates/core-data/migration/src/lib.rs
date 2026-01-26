pub use sea_orm_migration::prelude::*;

mod m2026_01_13_init;
mod m2026_01_26_add_auth0_sub_to_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m2026_01_13_init::Migration),
            Box::new(m2026_01_26_add_auth0_sub_to_users::Migration),
        ]
    }

    fn migration_table_name() -> DynIden {
        Alias::new("core_data_seaql_migrations").into_iden()
    }
}
