use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add a deleted_at column to clients for soft deletes
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE clients
                ADD COLUMN deleted_at TIMESTAMPTZ;
                "#,
            )
            .await?;

        // Add a deleted_at column to offices for soft deletes
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE offices 
                ADD COLUMN deleted_at TIMESTAMPTZ;
                "#,
            )
            .await?;

        // Add a deleted_at column to employees for soft deletes
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE employees 
                ADD COLUMN deleted_at TIMESTAMPTZ;
                "#,
            )
            .await?;

        // Add created_at and updated_at columns to clients, employees, offices
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE clients
                ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE offices
                ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE employees
                ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove the deleted_at column from clients
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE clients
                DROP COLUMN deleted_at;
                "#,
            )
            .await?;

        // Remove the deleted_at column from offices
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE offices
                DROP COLUMN deleted_at;
                "#,
            )
            .await?;

        // Remove the deleted_at column from employees
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE employees
                DROP COLUMN deleted_at;
                "#,
            )
            .await?;

        // Remove created_at and updated_at columns from clients, employees, offices
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE clients
                DROP COLUMN created_at,
                DROP COLUMN updated_at;
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE offices
                DROP COLUMN created_at,
                DROP COLUMN updated_at;
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE employees
                DROP COLUMN created_at,
                DROP COLUMN updated_at;
                "#,
            )
            .await?;

        Ok(())
    }
}
