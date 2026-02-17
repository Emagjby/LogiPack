use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add the column if it doesn't exist yet (nullable initially)
        manager
            .get_connection()
            .execute_unprepared(r#"ALTER TABLE users ADD COLUMN IF NOT EXISTS name TEXT;"#)
            .await?;

        // Backfill any NULL values
        manager
            .get_connection()
            .execute_unprepared(r#"UPDATE users SET name = 'User' WHERE name IS NULL;"#)
            .await?;

        // Enforce NOT NULL
        manager
            .get_connection()
            .execute_unprepared(r#"ALTER TABLE users ALTER COLUMN name SET NOT NULL;"#)
            .await?;

        manager
            .get_connection()
            .execute_unprepared(r#"ALTER TABLE employees DROP COLUMN IF EXISTS full_name;"#)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Re-add employees.full_name and backfill from users.name
        manager
            .get_connection()
            .execute_unprepared(r#"ALTER TABLE employees ADD COLUMN IF NOT EXISTS full_name TEXT;"#)
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"UPDATE employees SET full_name = u.name FROM users u WHERE employees.user_id = u.id;"#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(r#"ALTER TABLE employees ALTER COLUMN full_name SET NOT NULL;"#)
            .await?;

        // Drop users.name
        manager
            .get_connection()
            .execute_unprepared(r#"ALTER TABLE users DROP COLUMN IF EXISTS name;"#)
            .await?;

        Ok(())
    }
}
